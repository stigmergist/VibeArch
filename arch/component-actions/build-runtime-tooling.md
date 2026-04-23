# Build And Runtime Tooling Actions

## Navigation

- [Architecture Home](../README.md)
- [Next Steps](../next-steps.md)
- [Components](../components.md)
- Related components: [Frontend UI](frontend-ui.md), [Chat API](chat-api.md), [Connection Manager](connection-manager.md)

## Scope

Primary implementation: `frontend/package.json`, `frontend/vite.config.js`, `backend/requirements.txt`

## Current Risks And Gaps

- No Dockerfiles or container orchestration/dev packaging exists.
- No CI workflow exists.
- Runtime configuration is not environment-driven end to end.
- Observability and release/rollback procedures are undocumented.

## Recommended Actions

1. Add frontend/backend Dockerfiles and a simple local compose profile.
2. Add CI for build, test, and lint/type/syntax validation.
3. Introduce environment-driven config for socket URL and backend settings.
4. Add structured logging, metrics, and operational runbook content.
5. Define release and rollback procedure.

## Dependencies

- Supports [Frontend UI](frontend-ui.md) through env injection and frontend build tooling.
- Supports [Chat API](chat-api.md) and [Connection Manager](connection-manager.md) through deployment topology, observability, and testing.

## Open Questions

- Should the initial production target be VM-based or container-first?