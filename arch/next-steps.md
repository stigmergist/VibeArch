# Next Steps

This file is the top-level index for recommended architecture work. Component-specific detail lives in [component-details/frontend-ui.md](component-details/frontend-ui.md), [component-details/frontend-styling.md](component-details/frontend-styling.md), [component-details/chat-api.md](component-details/chat-api.md), [component-details/payload-validator.md](component-details/payload-validator.md), [component-details/connection-manager.md](component-details/connection-manager.md), and [component-details/build-runtime-tooling.md](component-details/build-runtime-tooling.md).

## High Priority

1. Add backend and frontend automated tests around chat protocol validation, error handling, disconnects, and reconnect behavior.
   Related components: [Chat API](component-details/chat-api.md), [Payload Validator](component-details/payload-validator.md), [Frontend UI](component-details/frontend-ui.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
2. Harden the auth/session lifecycle with logout, token expiry policy, origin restrictions, and automated coverage.
   Related components: [Chat API](component-details/chat-api.md), [Frontend UI](component-details/frontend-ui.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)

## Medium Priority

1. Add per-connection rate limiting to complete protocol abuse hardening.
   Related components: [Chat API](component-details/chat-api.md), [Payload Validator](component-details/payload-validator.md), [Connection Manager](component-details/connection-manager.md)
2. Define the scale path for broadcast fan-out before any multi-instance deployment.
   Related components: [Connection Manager](component-details/connection-manager.md), [Chat API](component-details/chat-api.md), [Build And Runtime Tooling](component-details/build-runtime-tooling.md)
3. Add production deployment baseline work: containerization, CI/CD, observability, and rollback procedure.
   Related components: [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Chat API](component-details/chat-api.md), [Frontend UI](component-details/frontend-ui.md)
4. Improve reconnect UX and accessibility support for inbound message announcements.
   Related components: [Frontend UI](component-details/frontend-ui.md), [Frontend Styling](component-details/frontend-styling.md)
5. Define backend settings/config conventions to match the now-documented frontend env contract.
   Related components: [Build And Runtime Tooling](component-details/build-runtime-tooling.md), [Chat API](component-details/chat-api.md)

## Completed Recently

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