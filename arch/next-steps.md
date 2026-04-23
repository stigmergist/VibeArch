# Next Steps

This file is the top-level index for recommended architecture work. Component-specific detail lives in [component-details/frontend-ui.md](component-details/frontend-ui.md), [component-details/frontend-styling.md](component-details/frontend-styling.md), [component-details/chat-api.md](component-details/chat-api.md), [component-details/payload-validator.md](component-details/payload-validator.md), [component-details/connection-manager.md](component-details/connection-manager.md), and [component-details/build-runtime-tooling.md](component-details/build-runtime-tooling.md).

## High Priority

1. Validate the deployed AWS handler path with smoke tests that mirror the now-working SAM-local flow, including the `$default` websocket route and DynamoDB-backed auth/session/connection behavior.
   Related components: [Chat API](component-details/chat-api.md), [Connection Manager](component-details/connection-manager.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
2. Add broader backend and frontend automated tests around chat protocol validation, error handling, disconnects, reconnect behavior, and the deployed serverless flow.
   Related components: [Chat API](component-details/chat-api.md), [Payload Validator](component-details/payload-validator.md), [Frontend UI](component-details/frontend-ui.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md)
3. Add CI/CD, observability, and rollback procedure for the serverless target now that the first DynamoDB-backed Lambda path exists.
   Related components: [AWS Serverless Platform](component-details/aws-serverless-platform.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Frontend UI](component-details/frontend-ui.md)

## Medium Priority

1. Add per-connection rate limiting to complete protocol abuse hardening.
   Related components: [Chat API](component-details/chat-api.md), [Payload Validator](component-details/payload-validator.md), [Connection Manager](component-details/connection-manager.md)
2. Improve reconnect UX and accessibility support for inbound message announcements.
   Related components: [Frontend UI](component-details/frontend-ui.md), [Frontend Styling](component-details/frontend-styling.md)
3. Define backend settings/config conventions to match the now-documented frontend env contract and the future AWS endpoint topology.
   Related components: [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Chat API](component-details/chat-api.md), [AWS Serverless Platform](component-details/aws-serverless-platform.md)

## Completed Recently

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