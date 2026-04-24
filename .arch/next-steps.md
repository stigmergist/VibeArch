# Next Steps

This file is the top-level index for recommended architecture work. Component-specific detail lives in [component-details/frontend-ui.md](component-details/frontend-ui.md), [component-details/frontend-styling.md](component-details/frontend-styling.md), [component-details/chat-api.md](component-details/chat-api.md), [component-details/payload-validator.md](component-details/payload-validator.md), [component-details/connection-manager.md](component-details/connection-manager.md), [component-details/build-runtime-tooling.md](component-details/build-runtime-tooling.md), and [component-details/aws-serverless-platform.md](component-details/aws-serverless-platform.md).

## Customer And Business Outcomes First

- Protect user trust at launch: prove the deployed AWS auth and chat path works end to end before calling production readiness.
- Reduce support burden and delivery risk: add broader automated coverage so regressions are caught before users see them.
- Improve recovery speed and operational confidence: add CI/CD gates, alarm routing, and rollback procedure on top of the new telemetry baseline.
- Keep durable history affordable and trustworthy: define retention, privacy, and capacity guardrails now that messages are stored and paged back into the UI.

## Priority Legend

- 🔴 High priority: immediate attention for customer trust or release confidence.
- 🟡 Medium priority: important quality and cost hardening after high-priority closure.
- 🟢 Completed recently: evidence of reduced risk.

## 🔴 High Priority

1. Prevent launch-time trust failures by validating the deployed AWS handler path with smoke tests that mirror the now-working SAM-local flow, including the `$default` websocket route and DynamoDB-backed auth/session/connection behavior.
   Related components: [Chat API](component-details/chat-api.md), [Connection Manager](component-details/connection-manager.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
2. Reduce customer-facing regressions and support churn by enforcing the growing backend and frontend regression suites in CI and extending them to the deployed serverless release flow.
   Related components: [Chat API](component-details/chat-api.md), [Payload Validator](component-details/payload-validator.md), [Frontend UI](component-details/frontend-ui.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md)
3. Improve release confidence and incident recovery by adding CI/CD, alarm routing, and rollback procedure for the serverless target now that dashboards and baseline alarms exist.
   Related components: [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Frontend UI](component-details/frontend-ui.md)
4. Define message-retention, privacy, and cost guardrails for persisted chat history before treating the new conversation replay path as production-ready.
   Related components: [Chat API](component-details/chat-api.md), [Frontend UI](component-details/frontend-ui.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md)

## 🟡 Medium Priority

1. Protect service quality and infrastructure cost by adding per-connection rate limiting to complete protocol abuse hardening.
   Related components: [Chat API](component-details/chat-api.md), [Payload Validator](component-details/payload-validator.md), [Connection Manager](component-details/connection-manager.md)
2. Improve retention and inclusive usability by deciding how reconnect should handle session continuity, draft preservation, and accessibility verification after the new bounded retry flow.
   Related components: [Frontend UI](component-details/frontend-ui.md), [Frontend Styling](component-details/frontend-styling.md)
3. Reduce deployment mistakes and onboarding friction by defining backend settings/config conventions to match the now-documented frontend env contract and the future AWS endpoint topology.
   Related components: [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Chat API](component-details/chat-api.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md)

## 🟢 Completed Recently

- 2026-04-24: Added persisted chat history plus lazy backward pagination so recent conversation restores on join and older pages load only on backward scroll.
   Related components: [Chat API](component-details/chat-api.md), [Frontend UI](component-details/frontend-ui.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md)
- 2026-04-24: Added explicit Lambda log retention plus a CloudWatch dashboard and baseline alarms to the AWS SAM stack so deployed auth/chat telemetry has an immediate operator view.
   Related components: [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
- 2026-04-24: Added structured JSON logs for auth, websocket, and broadcast paths, plus minimum service/SLO telemetry in the local health response.
   Related components: [Chat API](component-details/chat-api.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md)
- 2026-04-24: Added frontend Vitest coverage for reconnect and outbound payload shape, plus backend websocket tests for invalid payload handling and revoked-session behavior.
   Related components: [Frontend UI](component-details/frontend-ui.md), [Chat API](component-details/chat-api.md), [Payload Validator](component-details/payload-validator.md)
- 2026-04-24: Added bounded reconnect behavior and live-region status/message announcements in the frontend so transient disconnects no longer always force an immediate manual re-login.
   Related components: [Frontend UI](component-details/frontend-ui.md), [Frontend Styling](component-details/frontend-styling.md)
- 2026-04-24: Added a deployed AWS smoke harness that reuses the working SAM-local auth and websocket round-trip flow, with a Makefile target that can resolve `HttpApiUrl` and `WebSocketApiUrl` from CloudFormation outputs.
   Related components: [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Chat API](component-details/chat-api.md)
- 2026-04-24: Added explicit local startup preflight checks so missing DynamoDB Local or SAM build output fail early with actionable messages instead of surfacing as misleading browser-side errors.
   Related components: [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Chat API](component-details/chat-api.md)
- 2026-04-24: Added a one-command local Docker Compose profile (`docker compose up --build`) for frontend + direct Axum backend bring-up to reduce onboarding friction.
   Related components: [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Frontend UI](component-details/frontend-ui.md), [Chat API](component-details/chat-api.md)
- 2026-04-23: Added `backend/tests/aws_local_smoke.rs`, a `make sam-local-smoke` target, and browser-validated the Vite frontend against the SAM-local auth API plus websocket gateway.
   Related components: [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Frontend UI](component-details/frontend-ui.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
- 2026-04-23: Implemented DynamoDB-backed Lambda auth/session/connection handling and API Gateway fan-out in the shared `backend/` crate.
   Related components: [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Chat API](component-details/chat-api.md), [Connection Manager](component-details/connection-manager.md)
- 2026-04-23: Unified the AWS Lambda entry points into `backend/` so local Axum and AWS handlers now ship from one backend crate.
   Related components: [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Frontend UI](component-details/frontend-ui.md)
- 2026-04-23: Chose AWS serverless as the production target and documented the migration gap from the current process-local runtime.
   Related components: [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Chat API](component-details/chat-api.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
- 2026-04-23: Added fixed session expiry, `POST /auth/logout`, origin restrictions, and backend auth lifecycle tests.
   Related components: [Chat API](component-details/chat-api.md), [Frontend UI](component-details/frontend-ui.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
- 2026-04-23: Added baseline Dockerfiles and a local `docker compose` workflow for frontend/backend containers.
   Related components: [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Frontend UI](component-details/frontend-ui.md), [Chat API](component-details/chat-api.md)
- 2026-04-23: Added registration/login-backed session tokens and made `sender` server-owned after auth.
   Related components: [Chat API](component-details/chat-api.md), [Frontend UI](component-details/frontend-ui.md), [Payload Validator](component-details/payload-validator.md)
- 2026-04-23: Frontend socket configuration externalized via `VITE_CHAT_WS_URL` and documented with `.env.example`.
   Related components: [Frontend UI](component-details/frontend-ui.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
- 2026-04-23: Guaranteed disconnect/finally cleanup path added to the WebSocket handler.
   Related components: [Chat API](component-details/chat-api.md), [Connection Manager](component-details/connection-manager.md)

## Navigation

- [Architecture Wiki Home](README.md)
- [Components](components.md)
- [System Overview](system-overview.md)
- [Risks](risks.md)
- [Drift](drift.md)