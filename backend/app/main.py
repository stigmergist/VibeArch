from __future__ import annotations

import hashlib
import hmac
import json
import logging
import secrets
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import List

from fastapi import FastAPI, HTTPException, WebSocket, WebSocketDisconnect, status
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from starlette.websockets import WebSocketState

logger = logging.getLogger(__name__)

# --- Protocol constraints ---
MAX_FRAME_BYTES = 4_096        # max raw UTF-8 frame size
MAX_TEXT_CHARS = 1_000         # max message body after stripping whitespace
MAX_SENDER_CHARS = 48          # max display name length
MAX_USERNAME_CHARS = 24
MIN_PASSWORD_CHARS = 8


class RegisterRequest(BaseModel):
    username: str
    password: str
    displayName: str


class LoginRequest(BaseModel):
    username: str
    password: str


class SessionResponse(BaseModel):
    token: str
    username: str
    displayName: str


@dataclass
class UserRecord:
    username: str
    display_name: str
    password_salt: bytes
    password_hash: str


@dataclass
class SessionIdentity:
    token: str
    username: str
    display_name: str


def _normalize_username(raw: str) -> str:
    if not isinstance(raw, str):
        raise ValueError("Username must be a string.")

    username = raw.strip().lower()
    if not username:
        raise ValueError("Username is required.")
    if len(username) > MAX_USERNAME_CHARS:
        raise ValueError(f"Username must be at most {MAX_USERNAME_CHARS} characters.")
    if not all(character.isalnum() or character in {"-", "_"} for character in username):
        raise ValueError("Username may only contain letters, numbers, '-' and '_'.")

    return username


def _normalize_display_name(raw: str) -> str:
    if not isinstance(raw, str):
        raise ValueError("Display name must be a string.")

    display_name = raw.strip()
    if not display_name:
        raise ValueError("Display name is required.")
    if len(display_name) > MAX_SENDER_CHARS:
        raise ValueError(f"Display name must be at most {MAX_SENDER_CHARS} characters.")

    return display_name


def _validate_password(raw: str) -> str:
    if not isinstance(raw, str):
        raise ValueError("Password must be a string.")
    if len(raw) < MIN_PASSWORD_CHARS:
        raise ValueError(f"Password must be at least {MIN_PASSWORD_CHARS} characters.")

    return raw


def _hash_password(password: str, salt: bytes) -> str:
    return hashlib.pbkdf2_hmac("sha256", password.encode("utf-8"), salt, 100_000).hex()


class UserStore:
    def __init__(self) -> None:
        self.users: dict[str, UserRecord] = {}

    def register(self, username: str, password: str, display_name: str) -> UserRecord:
        normalized_username = _normalize_username(username)
        normalized_display_name = _normalize_display_name(display_name)
        validated_password = _validate_password(password)

        if normalized_username in self.users:
            raise ValueError("Username is already registered.")

        salt = secrets.token_bytes(16)
        user = UserRecord(
            username=normalized_username,
            display_name=normalized_display_name,
            password_salt=salt,
            password_hash=_hash_password(validated_password, salt),
        )
        self.users[normalized_username] = user
        return user

    def authenticate(self, username: str, password: str) -> UserRecord:
        normalized_username = _normalize_username(username)
        validated_password = _validate_password(password)
        user = self.users.get(normalized_username)

        if user is None:
            raise ValueError("Invalid username or password.")

        supplied_hash = _hash_password(validated_password, user.password_salt)
        if not hmac.compare_digest(supplied_hash, user.password_hash):
            raise ValueError("Invalid username or password.")

        return user


class SessionStore:
    def __init__(self) -> None:
        self.sessions: dict[str, SessionIdentity] = {}

    def create(self, user: UserRecord) -> SessionIdentity:
        token = secrets.token_urlsafe(32)
        identity = SessionIdentity(
            token=token,
            username=user.username,
            display_name=user.display_name,
        )
        self.sessions[token] = identity
        return identity

    def get(self, token: str) -> SessionIdentity | None:
        return self.sessions.get(token)


def _parse_and_validate(raw: str) -> dict | None:
    """
    Parse a raw WebSocket text frame and validate its shape.

    Returns a normalised dict ``{"sender": str, "text": str}`` on success,
    or ``None`` when the frame should be silently discarded.

    Raises ``ValueError`` with a user-safe description when the frame is
    structurally or semantically invalid and the client should receive an error.
    """
    if len(raw.encode("utf-8")) > MAX_FRAME_BYTES:
        raise ValueError(f"Frame exceeds maximum allowed size ({MAX_FRAME_BYTES} bytes).")

    try:
        data = json.loads(raw)
    except json.JSONDecodeError:
        raise ValueError("Malformed JSON — message was not delivered.")

    if not isinstance(data, dict):
        raise ValueError("Payload must be a JSON object.")

    if "sender" in data:
        raise ValueError("Field 'sender' is server-owned and must not be sent by clients.")

    # --- text field ---
    raw_text = data.get("text")
    if not isinstance(raw_text, str):
        raise ValueError("Field 'text' must be a string.")
    text = raw_text.strip()
    if not text:
        # empty text after strip — discard silently
        return None
    if len(text) > MAX_TEXT_CHARS:
        raise ValueError(f"Message text exceeds {MAX_TEXT_CHARS} characters.")
    return {"text": text}


class ConnectionManager:
    def __init__(self) -> None:
        self.connections: List[WebSocket] = []

    async def connect(self, websocket: WebSocket) -> None:
        await websocket.accept()
        self.connections.append(websocket)

    def disconnect(self, websocket: WebSocket) -> None:
        if websocket in self.connections:
            self.connections.remove(websocket)

    async def broadcast(self, payload: dict) -> None:
        dead_connections: List[WebSocket] = []
        for connection in self.connections:
            try:
                await connection.send_json(payload)
            except Exception:
                dead_connections.append(connection)

        for dead in dead_connections:
            self.disconnect(dead)


app = FastAPI(title="Simple Chat API")
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

manager = ConnectionManager()
users = UserStore()
sessions = SessionStore()


@app.get("/health")
def health() -> dict:
    return {"status": "ok"}


@app.post("/auth/register", response_model=SessionResponse, status_code=status.HTTP_201_CREATED)
def register(payload: RegisterRequest) -> SessionResponse:
    try:
        user = users.register(payload.username, payload.password, payload.displayName)
    except ValueError as exc:
        raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail=str(exc)) from exc

    session = sessions.create(user)
    return SessionResponse(
        token=session.token,
        username=session.username,
        displayName=session.display_name,
    )


@app.post("/auth/login", response_model=SessionResponse)
def login(payload: LoginRequest) -> SessionResponse:
    try:
        user = users.authenticate(payload.username, payload.password)
    except ValueError as exc:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail=str(exc)) from exc

    session = sessions.create(user)
    return SessionResponse(
        token=session.token,
        username=session.username,
        displayName=session.display_name,
    )


@app.websocket("/ws/chat")
async def chat_socket(websocket: WebSocket) -> None:
    token = websocket.query_params.get("token", "").strip()
    identity = sessions.get(token)
    if identity is None:
        await websocket.close(code=1008, reason="Authentication required.")
        return

    await manager.connect(websocket)

    join_message = {
        "type": "system",
        "text": f"{identity.display_name} joined the chat",
        "sentAt": datetime.now(timezone.utc).isoformat(),
    }
    await manager.broadcast(join_message)

    try:
        try:
            while True:
                raw = await websocket.receive_text()

                try:
                    validated = _parse_and_validate(raw)
                except ValueError as exc:
                    # Send a structured error back to the sender only; do not broadcast.
                    if websocket.application_state == WebSocketState.CONNECTED:
                        await websocket.send_json({
                            "type": "error",
                            "text": str(exc),
                            "sentAt": datetime.now(timezone.utc).isoformat(),
                        })
                    logger.warning("Rejected WebSocket payload: %s", exc)
                    continue

                if validated is None:
                    # Silently discard (e.g. blank message after strip).
                    continue

                message = {
                    "type": "message",
                    "sender": identity.display_name,
                    "text": validated["text"],
                    "sentAt": datetime.now(timezone.utc).isoformat(),
                }
                await manager.broadcast(message)

        except WebSocketDisconnect:
            # Clean disconnect path (explicit client close or server-initiated).
            pass

    except Exception as exc:
        # Unexpected runtime error: log context for debugging, monitoring, and alerting.
        logger.error(
            "Unexpected exception in chat socket handler: %s",
            exc,
            exc_info=True,
        )

    finally:
        # Guaranteed cleanup: remove from connection registry and notify others.
        if websocket in manager.connections:
            manager.disconnect(websocket)
            try:
                leave_message = {
                    "type": "system",
                    "text": f"{identity.display_name} left the chat",
                    "sentAt": datetime.now(timezone.utc).isoformat(),
                }
                await manager.broadcast(leave_message)
            except Exception as bcast_exc:
                logger.error(
                    "Failed to broadcast leave message: %s",
                    bcast_exc,
                    exc_info=True,
                )
