from __future__ import annotations

import json
from datetime import datetime, timezone
from typing import List

from fastapi import FastAPI, WebSocket, WebSocketDisconnect


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
        while True:
            raw = await websocket.receive_text()
            data = json.loads(raw)
            text = str(data.get("text", "")).strip()
            sender = str(data.get("sender", "Anonymous")).strip() or "Anonymous"

            if not text:
                continue

            message = {
                "type": "message",
                "sender": sender,
                "text": text,
                "sentAt": datetime.now(timezone.utc).isoformat(),
            }
            await manager.broadcast(message)

    except WebSocketDisconnect:
        manager.disconnect(websocket)
        leave_message = {
            "type": "system",
            "text": "A user left the chat",
            "sentAt": datetime.now(timezone.utc).isoformat(),
        }
        await manager.broadcast(leave_message)
