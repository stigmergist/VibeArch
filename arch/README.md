# Architecture Wiki

This wiki is the architecture source of truth for the repository.

## Navigation

- [System Overview](system-overview.md)
- [Components](components.md)
- [Next Steps](next-steps.md)
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

## Recommended Action Index

- Global architecture priorities live in [Next Steps](next-steps.md).
- Component-specific detail lives in:
	- [Frontend UI](component-details/frontend-ui.md)
	- [Frontend Styling](component-details/frontend-styling.md)
	- [Chat API](component-details/chat-api.md)
	- [Payload Validator](component-details/payload-validator.md)
	- [Connection Manager](component-details/connection-manager.md)
	- [Build And Runtime Tooling](component-details/build-runtime-tooling.md)

## Completed Recently

- 2026-04-23: Frontend websocket endpoint externalized via `VITE_CHAT_WS_URL` with documented local default/fallback.
- 2026-04-23: WebSocket resilience hardening completed with guaranteed disconnect/finally cleanup path and structured runtime error logging.
