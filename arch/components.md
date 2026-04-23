# Components

## Frontend UI (`frontend/src/App.jsx`)

Responsibilities:
- Open and maintain a WebSocket connection to backend chat endpoint.
- Render connection status, message list, and message composer.
- Send user-authored messages to server as JSON.
- Normalize sender fallback (`Guest`) and disable composer while disconnected.

Dependencies:
- Browser WebSocket API
- React state/effect hooks

## Frontend Styling (`frontend/src/styles.css`)

Responsibilities:
- Define visual theme and responsive layout.
- Provide motion and readability patterns for chat timeline.

Dependencies:
- Native CSS only

## Chat API (`backend/app/main.py`)

Responsibilities:
- Expose `GET /health` for simple health checks.
- Expose `WS /ws/chat` for bi-directional chat transport with guaranteed cleanup via try/except/finally.
- Enforce inbound payload protocol via `_parse_and_validate()` (see below).
- Route validation errors back to the originating client only; never broadcast them.
- Broadcast well-formed messages to all connected clients.
- Generate server-side event timestamps (`sentAt`).
- Catch `WebSocketDisconnect` cleanly and log unexpected runtime exceptions with full traceback context.

Dependencies:
- FastAPI
- Uvicorn runtime
- In-memory connection registry (`ConnectionManager`)
- Python standard library (`logging`)

## Payload Validator (`backend/app/main.py` — `_parse_and_validate`)

Responsibilities:
- Reject frames exceeding 4 096 bytes (UTF-8 encoded).
- Reject malformed JSON with a user-safe error message.
- Reject non-object JSON (e.g. bare strings or arrays).
- Reject `text` that is absent, non-string, blank after strip, or longer than 1 000 characters.
- Reject `sender` that is non-string; silently truncate to 48 characters; default to `"Anonymous"`.
- Return `None` for silently discarded frames (blank text).
- Raise `ValueError` with a user-safe description for all other invalid payloads.

Dependencies:
- Python standard library (`json`)

## Connection Manager (`backend/app/main.py`)

Responsibilities:
- Track connected WebSocket clients.
- Fan-out/broadcast messages and clean dead connections.

Dependencies:
- FastAPI `WebSocket`

## Build And Runtime Tooling

Frontend tooling (`frontend/package.json`):
- Vite dev/build/preview scripts.
- React plugin for Vite.

Backend dependency manifest (`backend/requirements.txt`):
- FastAPI and Uvicorn standard extras only.
