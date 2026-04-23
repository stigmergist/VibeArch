# Build And Runtime Tooling

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Frontend UI](frontend-ui.md), [Chat API](chat-api.md), [Connection Manager](connection-manager.md)

## Scope

Primary implementation: `compose.yaml`, `frontend/Dockerfile`, `frontend/nginx.conf`, `frontend/package.json`, `frontend/vite.config.js`, `backend/Dockerfile`, `backend/requirements.txt`

## Responsibilities

- Define frontend and backend local runtime/tooling dependencies.
- Provide the base surface for packaging, containerized local runs, CI, observability rollout, and environment configuration.

## Dependencies

- [Frontend UI](frontend-ui.md)
- [Chat API](chat-api.md)
- [Connection Manager](connection-manager.md)

## Risks And Gaps

- No CI workflow exists.
- Local container packaging exists, but there are no production deployment manifests or runtime hardening conventions yet.
- Frontend runtime configuration is now environment-driven for the socket URL, and auth endpoint discovery is derived from that same value, but backend settings and deployment injection conventions are still incomplete.
- Observability and release/rollback procedures are undocumented.

## Recommended Actions

1. Add CI for build, test, and lint/type/syntax validation.
2. Introduce backend environment-driven settings and document deployment-time injection for `VITE_CHAT_WS_URL`, allowed origins, and session policy.
3. Add production deployment manifests and container runtime hardening beyond the local compose profile.
4. Add structured logging, metrics, and operational runbook content.
5. Define release and rollback procedure.

## Open Questions

- Which managed container platform should be the first production target on top of the local compose baseline?