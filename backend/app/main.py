from __future__ import annotations

import json
import logging
from datetime import datetime, timezone
from typing import List

from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from starlette.websockets import WebSocketState

logger = logging.getLogger(__name__)

# --- Protocol constraints ---
MAX_FRAME_BYTES = 4_096        # max raw UTF-8 frame size
MAX_TEXT_CHARS = 1_000         # max message body after stripping whitespace
MAX_SENDER_CHARS = 48          # max display name length


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

    # --- sender field ---
    raw_sender = data.get("sender", "")
    if not isinstance(raw_sender, str):
        raise ValueError("Field 'sender' must be a string.")
    sender = raw_sender.strip()[:MAX_SENDER_CHARS] or "Anonymous"

    return {"sender": sender, "text": text}


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
manager = ConnectionManager()


@app.get("/health")
def health() -> dict:
    return {"status": "ok"}


@app.websocket("/ws/chat")
async def chat_socket(websocket: WebSocket) -> None:
    await manager.connect(websocket)

    join_message = {
        "type": "system",
        "text": "A user joined the chat",
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
                    "sender": validated["sender"],
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
                    "text": "A user left the chat",
                    "sentAt": datetime.now(timezone.utc).isoformat(),
                }
                await manager.broadcast(leave_message)
            except Exception as bcast_exc:
                logger.error(
                    "Failed to broadcast leave message: %s",
                    bcast_exc,
                    exc_info=True,
                )
