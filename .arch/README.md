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

- Code zone: repository root except `.arch/`
- Knowledge zone: `.arch/`

## Current System Snapshot

- Frontend: React 18 + Vite app in `frontend/`
- Backend: shared Rust crate in `backend/` that contains the Lambda handlers, the local websocket gateway shim, and the shared validation/auth logic reused across local and AWS paths
- AWS scaffold: `infra/aws/template.yaml` plus `backend/src/aws_lambda.rs` and the Lambda binaries in `backend/src/bin/`
- Auth/session: `POST /auth/register`, `POST /auth/login`, `POST /auth/logout`, fixed-lifetime bearer sessions, and token-authenticated WebSocket chat on `/ws/chat`
- Persistence: the supported backend path persists users, sessions, chat history, and connection IDs in DynamoDB or DynamoDB Local
- Supported local backend path: DynamoDB Local + `sam local start-api` + `cargo run --bin local_gateway`
- Additional local convenience path: `docker compose up --build` runs frontend + direct Axum backend for one-command bring-up
- Production target direction: AWS serverless with static frontend hosting plus Lambda-backed auth and chat integrations

## NFR And Deployability Snapshot

- NFR status summary: 🟢 good in flexibility/input validation/modularity, 🟡 watch in availability/resilience/performance/scalability/security/manageability/portability/cost/robustness/reliability/fault tolerance/observability/testability/maintainability/privacy and data protection/usability/accessibility.
- Deployability today: stronger for real chat continuity because the supported AWS-local development path now persists recent conversation history in the same DynamoDB-backed handler flow used for AWS. The stack also has baseline CloudWatch monitoring, but production still lacks CI/CD, alarm routing, secrets handling, retention policy decisions for stored messages, and repeated deployed validation in release operations.
- Details and evidence: see `system-overview.md`, `risks.md`, and `drift.md`.

## Scan First (Traffic Light)

- 🔴 Act now: deployed AWS validation, CI/CD enforcement, and message-retention/alarm-routing gaps are the main launch-confidence risks.
- 🟡 Watch closely: reliability and delivery-speed still depend on release enforcement for deployed smoke checks, tuned thresholds, and history-pagination regression coverage.
- 🟢 Stable base: input validation, modular boundaries, server-owned sender identity, persisted recent conversation history, and baseline service monitoring are in a good state.

## Recommended Action Index

- Global architecture priorities live in [Next Steps](next-steps.md).
- Component-specific detail lives in:
	- [Frontend UI](component-details/frontend-ui.md)
	- [Frontend Styling](component-details/frontend-styling.md)
	- [Chat API](component-details/chat-api.md)
	- [Payload Validator](component-details/payload-validator.md)
	- [Connection Manager](component-details/connection-manager.md)
	- [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
	- [AWS Serverless Platform](component-details/aws-serverless-platform.md)

## Completed Recently

- 2026-04-24: Added persisted chat history plus lazy backward pagination so recent conversation is restored on join and older pages load only when the user scrolls upward.
- 2026-04-24: Added CloudWatch dashboard and baseline alarms for the AWS SAM stack, plus explicit Lambda log-group retention for deployed monitoring.
- 2026-04-24: Added structured JSON tracing across local and Lambda backend entrypoints, plus minimum service/SLO telemetry in the local health response.
- 2026-04-24: Added bounded frontend reconnect behavior plus frontend and backend regression tests for reconnect, protocol validation, and revoked-session handling.
- 2026-04-24: Added a deployed AWS smoke harness that reuses the local SAM auth and websocket round-trip flow, with a Makefile target that can resolve stack outputs from CloudFormation.
- 2026-04-24: Hardened the SAM-local startup path in `backend/Makefile` with explicit DynamoDB reachability and SAM build-artifact checks so local dependency failures stop early with actionable messages.
- 2026-04-23: Validated the SAM-local auth path end to end from the browser, added a local websocket smoke test, and removed remaining local-runtime assumptions from the shared handler path.
- 2026-04-23: Chose AWS serverless as the production target and documented the migration gap from the current process-local Axum runtime.
- 2026-04-23: Implemented the first DynamoDB-backed Lambda auth/session/connection flow and added a SAM local workflow for the shared backend crate.
- 2026-04-23: Unified the AWS Lambda entry points into the main `backend/` crate so local and AWS backend code now live in one codebase.
- 2026-04-23: Replaced the Python backend with a Rust Axum backend while preserving the existing auth and WebSocket contract.
- 2026-04-23: Added fixed-lifetime sessions, logout, origin restrictions, and backend auth lifecycle tests.
- 2026-04-23: Added baseline Dockerfiles plus a local `docker compose` workflow for the container-first deployment path.
- 2026-04-23: Added in-memory registration/login session tokens and made chat sender identity server-owned after auth.
- 2026-04-23: Frontend websocket endpoint externalized via `VITE_CHAT_WS_URL` with documented local default/fallback.
- 2026-04-23: WebSocket resilience hardening completed with guaranteed disconnect/finally cleanup path and structured runtime error logging.
