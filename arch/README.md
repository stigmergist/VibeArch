# Architecture Wiki

This wiki is the architecture source of truth for the repository.

## Navigation

- [System Overview](system-overview.md)
- [Components](components.md)
- [Data Flow](data-flow.md)
- [Decisions](decisions.md)
- [Risks](risks.md)
- [Drift](drift.md)
- [Change Log](change-log.md)

## Scope

- Code zone: repository root except `arch/`
- Knowledge zone: `arch/`

## Current System Snapshot

- Frontend: React 18 + Vite app in `frontend/`
- Backend: FastAPI + Uvicorn app in `backend/`
- Transport: WebSocket chat on `/ws/chat`
- Persistence: none (ephemeral, in-memory only)

## NFR And Deployability Snapshot

- NFR status summary: good in flexibility, watch in performance/scalability/portability/cost/manageability, weak in security, availability, and resilience.
- Deployability today: strong for local development, partial for cloud VM/manual deploy, not production-ready for managed/containerized operation yet.
- Details and evidence: see `system-overview.md`, `risks.md`, and `drift.md`.

## Prioritized Next Architecture Work

1. Introduce protocol hardening for incoming WebSocket payloads (shape, size, malformed JSON handling).
2. Externalize frontend socket configuration via Vite environment variables.
3. Add backend and frontend automated tests around chat contract and reconnect behavior.
4. Add an identity/auth strategy to prevent sender impersonation.
5. Define scale path for broadcast fan-out (Redis pub/sub or equivalent) before multi-instance deployment.
6. Add production deployment baseline: containerization, CI/CD, observability, and rollback path.
