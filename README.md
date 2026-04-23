# Simple Chat App (React + Python)

A minimal real-time chat application with:
- React frontend (Vite)
- Python backend (FastAPI + WebSocket)

## Project structure

- `frontend/` React chat UI
- `backend/` FastAPI server with chat WebSocket endpoint
- `arch/` architecture wiki and decision log

## Run locally

### 1. Start backend

```bash
cd backend
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
uvicorn app.main:app --reload --port 8000
```

Backend endpoints:
- Health check: `http://localhost:8000/health`
- Register: `http://localhost:8000/auth/register`
- Login: `http://localhost:8000/auth/login`
- Chat socket: `ws://localhost:8000/ws/chat`

### 2. Start frontend (new terminal)

```bash
cd frontend
npm install
cp .env.example .env
npm run dev
```

Open `http://localhost:5173`.

Frontend environment contract:
- `VITE_CHAT_WS_URL`: full websocket URL used by the browser client.
- Local default/example: `ws://localhost:8000/ws/chat`
- Production example: `wss://chat.example.com/ws/chat`
- The frontend derives its auth API base URL from this value by switching `ws` -> `http` and replacing the trailing `/ws/chat` with `/auth`.
- Deployment contract: the configured websocket URL must point at the same backend that serves `POST /auth/register` and `POST /auth/login`.

## How it works

- User creates an account or signs in from the frontend; the backend returns an in-memory session token.
- Frontend opens a WebSocket connection to `VITE_CHAT_WS_URL?token=...` and falls back to `ws://localhost:8000/ws/chat` when the env var is unset.
- Client sends `{ text }` payloads only.
- Backend authenticates the socket from the session token and stamps `sender` from the authenticated identity.
- Backend rejects any client payload that tries to send its own `sender` field.
- Backend broadcasts each valid message to all connected clients.
- Join/leave events are sent as system messages.
