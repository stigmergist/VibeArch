# System Overview

## Goals

- Provide a simple real-time chat experience for multiple browser clients.
- Keep implementation lightweight and easy to run locally.

## Boundaries

- Frontend runtime: browser, served by Vite dev server (`frontend/`).
- Backend runtime: FastAPI server (`backend/`).
- Persistence: none (in-memory only).
- Wire protocol: unversioned JSON messages over a single WebSocket endpoint.

## Runtime Context Narrative

- Users open the React app and connect over WebSocket to the backend.
- Backend tracks active socket connections in memory.
- Incoming client messages are broadcast to all active clients.
- Backend also emits system join/leave messages.
- Health check is exposed at `GET /health`.

## Major Runtime Concerns

- Connection lifecycle management for disconnect/reconnect.
- Input validation for incoming payload shape and size.
- Malformed JSON handling in socket loop (currently no explicit guard).
- No authentication or authorization in current scope.
- No data persistence or chat history retention.
- Single-process memory model limits horizontal scalability.

## Assumptions

- Development environment uses `localhost` with frontend on `5173` and backend on `8000`.
- Frontend and backend are launched separately during local development.
- Message timestamps are generated server-side in UTC ISO-8601 format.
