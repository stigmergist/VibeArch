# Components

This page is the high-level component map for the repository. Per-component detail lives in [component-details/frontend-ui.md](component-details/frontend-ui.md), [component-details/frontend-styling.md](component-details/frontend-styling.md), [component-details/chat-api.md](component-details/chat-api.md), [component-details/payload-validator.md](component-details/payload-validator.md), [component-details/connection-manager.md](component-details/connection-manager.md), and [component-details/build-runtime-tooling.md](component-details/build-runtime-tooling.md).

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
- FastAPI websocket and health endpoint implementation in `backend/app/main.py`.
- Orchestrates registration/login, session lookup, validation, connection lifecycle handling, and outbound message broadcast.

High-level relationships:
- Depends on [Payload Validator](component-details/payload-validator.md) for inbound protocol hardening.
- Depends on [Connection Manager](component-details/connection-manager.md) for active-client tracking and fan-out.
- Serves [Frontend UI](component-details/frontend-ui.md) as the current transport boundary.
- Relies on [Build And Runtime Tooling](component-details/build-runtime-tooling.md) for deployment and observability rollout.

## [Payload Validator](component-details/payload-validator.md)

Summary:
- Protocol-hardening helper `_parse_and_validate()` in `backend/app/main.py`.
- Encodes message size, shape, type, and normalization rules before messages enter the chat flow.

High-level relationships:
- Feeds sanitized data into [Chat API](component-details/chat-api.md).
- Influences failure and rejection behavior seen by [Frontend UI](component-details/frontend-ui.md).

## [Connection Manager](component-details/connection-manager.md)

Summary:
- Process-local websocket registry and broadcast helper inside `backend/app/main.py`.
- Owns fan-out behavior and dead-connection cleanup within a single process.

High-level relationships:
- Used by [Chat API](component-details/chat-api.md) for connect/disconnect/broadcast flow.
- Constrains scalability and failover until [Build And Runtime Tooling](component-details/build-runtime-tooling.md) introduces a shared adapter path.

## [Build And Runtime Tooling](component-details/build-runtime-tooling.md)

Summary:
- Local build and runtime surface spanning `frontend/package.json`, `frontend/vite.config.js`, and `backend/requirements.txt`.
- Governs how configuration, packaging, CI, and operational tooling will be introduced.

High-level relationships:
- Enables environment/configuration needs for [Frontend UI](component-details/frontend-ui.md) and [Chat API](component-details/chat-api.md).
- Enables deployment, observability, and scale-path work for [Connection Manager](component-details/connection-manager.md).

## Navigation

- [Architecture Home](README.md)
- [Next Steps](next-steps.md)
- [System Overview](system-overview.md)
