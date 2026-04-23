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

- NFR status summary: 🟢 good in flexibility/input validation/modularity, 🟡 watch in availability/resilience/performance/scalability/manageability/portability/cost/robustness/reliability/fault tolerance/testability/maintainability/privacy and data protection/usability/accessibility, 🔴 weak in security/observability.
- Deployability today: strong for local development, partial for cloud VM/manual deploy, not production-ready for managed/containerized operation yet.
- Details and evidence: see `system-overview.md`, `risks.md`, and `drift.md`.

## Prioritized Next Architecture Work

1. Externalize frontend socket configuration via Vite environment variables (`VITE_CHAT_WS_URL`).
2. Add backend and frontend automated tests around chat protocol validation, error handling, and reconnect behavior.
3. Add an identity/auth strategy to prevent sender impersonation and make sender identity server-owned.
4. Add per-connection rate limiting to complete protocol abuse hardening beyond current payload size/shape checks.
5. Define scale path for broadcast fan-out (Redis pub/sub or equivalent) before multi-instance deployment.
6. Add production deployment baseline: containerization, CI/CD, observability, and rollback path.

## Completed Recently

- 2026-04-23: WebSocket resilience hardening completed with guaranteed disconnect/finally cleanup path and structured runtime error logging.
