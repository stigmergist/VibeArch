# AWS Serverless Platform

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Build And Runtime Tooling](build-runtime-tooling.md), [Chat API](chat-api.md), [Connection Manager](connection-manager.md), [Frontend UI](frontend-ui.md)

## Scope

Primary implementation target: `infra/aws/README.md`, `infra/aws/template.yaml`, `infra/aws/env.local.json`, `backend/src/aws_lambda.rs`, `backend/src/bin/*.rs`, `backend/tests/aws_local_smoke.rs`

## Responsibilities

- Define the AWS production runtime boundary.
- Provide the infrastructure and shared-handler implementation for the AWS production target.
- Map frontend static hosting to S3 + CloudFront.
- Map auth behavior to Lambda-backed HTTP APIs.
- Map chat behavior to API Gateway WebSocket routes plus Lambda handlers.
- Move user/session/connection state to DynamoDB-backed persistence.

## Dependencies

- AWS Lambda
- API Gateway HTTP API
- API Gateway WebSocket API
- DynamoDB
- S3 and CloudFront
- [Build And Runtime Tooling](build-runtime-tooling.md)
- [Chat API](chat-api.md)

## Risks And Gaps

- API Gateway WebSocket pricing still includes connection-minute cost, so “pay only when used” is approximate rather than zero-idle.
- The shared Lambda path now persists auth/session/connection state, performs API Gateway fan-out, and runs locally through SAM plus the websocket gateway shim, but CI/CD, secrets management, observability, and deployed AWS validation are still incomplete.

## Recommended Actions

1. Validate the deployed AWS path with smoke tests that mirror the now-working SAM-local auth and websocket flow, including the `$default` websocket route.
2. Finalize production env vars and public domain topology for `VITE_CHAT_WS_URL` and `VITE_AUTH_BASE_URL`.
3. Add CloudWatch logs, metrics, alarms, and rollout/rollback procedures.
4. Add CI/CD and secrets handling for the AWS deployment path.

## Recent Evidence

- `backend/tests/aws_local_smoke.rs` now exercises register plus websocket message round-trip against the local SAM auth API and local websocket gateway.
- Browser validation succeeded against `http://127.0.0.1:3000/auth/*`, `ws://127.0.0.1:3001/ws/chat`, and the Vite frontend on `http://127.0.0.1:5173`.
- `backend/src/aws_lambda.rs` now normalizes SAM-local logical table names and local DynamoDB/websocket defaults so the shared handlers run through the same code path locally and in AWS.
- `backend/Makefile`, `compose.aws-local.yaml`, and `infra/aws/env.local.json` now define the supported local AWS workflow rather than a separate backend runtime.

## Open Questions

- Should the frontend use one shared domain for both HTTP auth and WebSocket traffic in production?
- Should sessions remain opaque bearer tokens in DynamoDB, or move to JWT + revocation tracking?
- Is single-room chat still the desired scope for the AWS deployment?