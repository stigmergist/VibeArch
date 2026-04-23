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
- Chat socket: `ws://localhost:8000/ws/chat`

### 2. Start frontend (new terminal)

```bash
cd frontend
npm install
npm run dev
```

Open `http://localhost:5173`.

## How it works

- Frontend opens a WebSocket connection to `ws://localhost:8000/ws/chat`.
- Client sends `{ sender, text }` payloads.
- Backend broadcasts each valid message to all connected clients.
- Join/leave events are sent as system messages.
