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
- Expose `WS /ws/chat` for bi-directional chat transport.
- Validate incoming message payload basics and broadcast outbound events.
- Generate server-side event timestamps (`sentAt`).

Dependencies:
- FastAPI
- Uvicorn runtime
- In-memory connection registry (`ConnectionManager`)

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
