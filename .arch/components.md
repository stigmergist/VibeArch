# Components

This page is the high-level component map for the repository. Per-component detail lives in [component-details/frontend-ui.md](component-details/frontend-ui.md), [component-details/frontend-styling.md](component-details/frontend-styling.md), [component-details/chat-api.md](component-details/chat-api.md), [component-details/payload-validator.md](component-details/payload-validator.md), [component-details/connection-manager.md](component-details/connection-manager.md), [component-details/build-runtime-tooling.md](component-details/build-runtime-tooling.md), and [component-details/aws-serverless-platform.md](component-details/aws-serverless-platform.md).

## [Frontend UI](component-details/frontend-ui.md)

Summary:
- Browser-facing auth and chat experience implemented in `frontend/src/App.jsx`.
- Relies on [Chat API](component-details/chat-api.md) for the message contract and on [Frontend Styling](component-details/frontend-styling.md) for presentation.

High-level relationships:
- Consumes the register/login HTTP endpoints and websocket protocol exposed by [Chat API](component-details/chat-api.md).
- Shares UX/accessibility concerns with [Frontend Styling](component-details/frontend-styling.md).
- Depends on [Build And Runtime Tooling](component-details/build-runtime-tooling.md) for environment configuration and test automation.

## [Frontend Styling](component-details/frontend-styling.md)

Summary:
- Visual system and responsive layout for the chat UI in `frontend/src/styles.css`.
- Closely coupled to [Frontend UI](component-details/frontend-ui.md) state and semantics.

High-level relationships:
- Styles the states emitted by [Frontend UI](component-details/frontend-ui.md).
- Carries the main accessibility/contrast presentation concerns for the browser layer.

## [Chat API](component-details/chat-api.md)

Summary:
- Shared auth and chat orchestration surface spanning `backend/src/aws_lambda.rs` and shared helpers in `backend/src/lib.rs`.
- Owns register/login/logout, session lookup, payload validation, connection lifecycle handling, and outbound message broadcast across local and AWS execution paths.

High-level relationships:
- Depends on [Payload Validator](component-details/payload-validator.md) for inbound protocol hardening.
- Depends on [Connection Manager](component-details/connection-manager.md) for active-client tracking and fan-out.
- Serves [Frontend UI](component-details/frontend-ui.md) as the current transport boundary.
- Relies on [Build And Runtime Tooling](component-details/build-runtime-tooling.md) for deployment and observability rollout.

## [Payload Validator](component-details/payload-validator.md)

Summary:
- Shared protocol-hardening helper in `backend/src/runtime_contract.rs`.
- Encodes message size, shape, type, normalization, and session TTL policy used by both local and AWS runtime paths.

High-level relationships:
- Feeds sanitized data into [Chat API](component-details/chat-api.md).
- Influences failure and rejection behavior seen by [Frontend UI](component-details/frontend-ui.md).

## [Connection Manager](component-details/connection-manager.md)

Summary:
- DynamoDB-backed connection tracking plus the local websocket gateway peer map used to emulate API Gateway Management API behavior.
- Owns fan-out behavior and stale/dead-connection cleanup across the supported local and AWS-oriented paths.

High-level relationships:
- Used by [Chat API](component-details/chat-api.md) for connect/disconnect/broadcast flow.
- Constrains scalability and failover until [Build And Runtime Tooling](component-details/build-runtime-tooling.md) introduces a shared adapter path.

## [Build And Runtime Tooling](component-details/build-runtime-tooling.md)

Summary:
- Local build and runtime surface spanning the SAM workflow, DynamoDB Local compose file, frontend tooling, backend Cargo manifests, and AWS infrastructure docs.
- Governs configuration, packaging, CI, and operational tooling for the supported AWS-local and AWS deployment paths.

High-level relationships:
- Enables environment/configuration needs for [Frontend UI](component-details/frontend-ui.md) and [Chat API](component-details/chat-api.md).
- Enables deployment, observability, and scale-path work for [Connection Manager](component-details/connection-manager.md).

## [AWS Serverless Platform](component-details/aws-serverless-platform.md)

Summary:
- Target production deployment shape for AWS pay-per-use hosting.
- Current scaffold lives in `infra/aws/template.yaml` and the shared `backend/` crate.
- Replaces the current always-on backend process assumption with API Gateway, Lambda, and DynamoDB responsibilities.

High-level relationships:
- Pulls configuration and deployment concerns out of [Build And Runtime Tooling](component-details/build-runtime-tooling.md) into an AWS-specific runtime model.
- Forces [Chat API](component-details/chat-api.md) and [Connection Manager](component-details/connection-manager.md) to move from process-local behavior to event-driven and persistent state handling.
- Defines the production hosting boundary consumed by [Frontend UI](component-details/frontend-ui.md).

## Navigation

- [Architecture Home](README.md)
- [Next Steps](next-steps.md)
- [System Overview](system-overview.md)
