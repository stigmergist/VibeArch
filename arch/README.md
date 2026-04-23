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
- Auth/session: `POST /auth/register`, `POST /auth/login`, then token-authenticated WebSocket chat on `/ws/chat`
- Persistence: none (ephemeral, in-memory only)

## NFR And Deployability Snapshot

- NFR status summary: 🟢 good in flexibility/input validation/modularity, 🟡 watch in availability/resilience/performance/scalability/security/manageability/portability/cost/robustness/reliability/fault tolerance/testability/maintainability/privacy and data protection/usability/accessibility, 🔴 weak in observability.
- Deployability today: strong for local development and local containerized runs, but not production-ready for the chosen container-first deployment target because production manifests, CI, and operational controls are still missing.
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

- 2026-04-23: Added baseline Dockerfiles plus a local `docker compose` workflow for the container-first deployment path.
- 2026-04-23: Added in-memory registration/login session tokens and made chat sender identity server-owned after auth.
- 2026-04-23: Frontend websocket endpoint externalized via `VITE_CHAT_WS_URL` with documented local default/fallback.
- 2026-04-23: WebSocket resilience hardening completed with guaranteed disconnect/finally cleanup path and structured runtime error logging.
