# Decisions (ADR Lite)

## ADR-001: Use WebSocket for Chat Transport

- Status: accepted
- Date: 2026-04-23
- Decision: Implement chat communication over a single WebSocket endpoint (`/ws/chat`).
- Rationale: Real-time bi-directional messaging with minimal protocol overhead.
- Consequences: Must manage connection lifecycle and handle dropped sockets.

## ADR-002: Use FastAPI for Backend

- Status: accepted
- Date: 2026-04-23
- Decision: Build backend with FastAPI + Uvicorn.
- Rationale: Simple async WebSocket support and low setup complexity.
- Consequences: Runtime state is process-local unless externalized.

## ADR-003: Keep Chat State In-Memory

- Status: accepted
- Date: 2026-04-23
- Decision: Do not persist messages/users in v1.
- Rationale: Keep project simple for learning/prototyping.
- Consequences: No history across refresh/restart; no horizontal scaling safety.

## ADR-004: Hard-Code WebSocket Endpoint For Local-First v1

- Status: accepted (temporary)
- Date: 2026-04-23
- Decision: Frontend uses a direct constant (`ws://localhost:8000/ws/chat`) in `frontend/src/App.jsx`.
- Rationale: Minimize configuration overhead during initial bring-up.
- Consequences: Environment portability is reduced; production readiness requires config externalization.

## ADR-005: Keep Message Contract Unversioned In v1

- Status: accepted (temporary)
- Date: 2026-04-23
- Decision: Message envelope has no explicit schema version/type registry beyond current fields.
- Rationale: Move quickly with a minimal chat protocol.
- Consequences: Contract evolution risk increases as features are added.

## ADR-006: Add Protocol Validation And Error Guards

- Status: proposed
- Date: 2026-04-23
- Decision: Introduce strict payload validation (shape + max length) and explicit malformed JSON handling in the socket loop.
- Rationale: Prevent handler crashes, abusive payloads, and ambiguous client behavior.
- Consequences: Slight complexity increase, significant reliability and security improvement.
