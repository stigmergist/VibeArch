# Build And Runtime Tooling

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Frontend UI](frontend-ui.md), [Chat API](chat-api.md), [Connection Manager](connection-manager.md)

## Scope

Primary implementation: `frontend/package.json`, `frontend/vite.config.js`, `backend/requirements.txt`

## Responsibilities

- Define frontend and backend local runtime/tooling dependencies.
- Provide the base surface for packaging, CI, observability rollout, and environment configuration.

## Dependencies

- [Frontend UI](frontend-ui.md)
- [Chat API](chat-api.md)
- [Connection Manager](connection-manager.md)

## Risks And Gaps

- No Dockerfiles or container orchestration/dev packaging exists.
- No CI workflow exists.
- Frontend runtime configuration is now environment-driven for the socket URL, but backend settings and deployment injection conventions are still incomplete.
- Observability and release/rollback procedures are undocumented.

## Recommended Actions

1. Add frontend/backend Dockerfiles and a simple local compose profile.
2. Add CI for build, test, and lint/type/syntax validation.
3. Introduce backend environment-driven settings and document deployment-time injection for `VITE_CHAT_WS_URL`.
4. Add structured logging, metrics, and operational runbook content.
5. Define release and rollback procedure.

## Open Questions

- Should the initial production target be VM-based or container-first?