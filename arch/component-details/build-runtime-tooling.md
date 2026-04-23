# Build And Runtime Tooling

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Frontend UI](frontend-ui.md), [Chat API](chat-api.md), [Connection Manager](connection-manager.md)

## Scope

Primary implementation: `compose.aws-local.yaml`, `frontend/Dockerfile`, `frontend/nginx.conf`, `frontend/package.json`, `frontend/vite.config.js`, `frontend/.env.example`, `backend/Dockerfile`, `backend/Cargo.toml`, `backend/Cargo.lock`, `backend/Makefile`, `backend/src/lib.rs`, `backend/src/bin/*.rs`, `infra/aws/template.yaml`, `infra/aws/env.local.json`

## Responsibilities

- Define the supported AWS-local frontend and backend runtime/tooling dependencies.
- Provide the base surface for packaging, containerized local runs, CI, observability rollout, and environment configuration.

## Dependencies

- [Frontend UI](frontend-ui.md)
- [Chat API](chat-api.md)
- [Connection Manager](connection-manager.md)

## Risks And Gaps

- No CI workflow exists.
- Local container packaging exists, and there is now a first AWS SAM scaffold built from the same backend crate used locally, but production deployment automation, secrets handling, and runtime hardening conventions are still incomplete.
- Frontend runtime configuration is environment-driven for `VITE_CHAT_WS_URL` and `VITE_AUTH_BASE_URL`, and backend session/origin policy is environment-driven via `ALLOWED_ORIGINS` and `SESSION_TTL_SECONDS`, but broader deployment injection conventions are still incomplete.
- Observability and release/rollback procedures are undocumented.

## Recommended Actions

1. Add CI for `cargo test`, frontend build validation, and future SAM validation/deploy checks.
2. Extend and document deployment-time injection for `VITE_CHAT_WS_URL`, `VITE_AUTH_BASE_URL`, `ALLOWED_ORIGINS`, `SESSION_TTL_SECONDS`, and AWS table/runtime settings.
3. Finish the AWS deployment scaffold with deploy automation, secrets handling, and runtime hardening.
4. Add structured logging, metrics, and operational runbook content.
5. Define release and rollback procedure.

## Open Questions

- Should `cargo-lambda` remain the standard SAM build path, or should the repo standardize on a containerized SAM build for CI portability?