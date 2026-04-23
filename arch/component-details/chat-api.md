# Chat API

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Payload Validator](payload-validator.md), [Connection Manager](connection-manager.md), [Frontend UI](frontend-ui.md), [Build And Runtime Tooling](build-runtime-tooling.md)

## Scope

Primary implementation: `backend/app/main.py`

## Responsibilities

- Expose `GET /health` for basic health checks.
- Expose `POST /auth/register` and `POST /auth/login` for credential-based session creation.
- Expose `WS /ws/chat` for bidirectional chat transport.
- Coordinate session lookup, payload validation, broadcast flow, lifecycle cleanup, and server-side timestamps.

## Dependencies

- FastAPI
- Uvicorn runtime
- Python `hashlib`, `hmac`, and `secrets` for password hashing and token generation
- [Payload Validator](payload-validator.md)
- [Connection Manager](connection-manager.md)
- [Build And Runtime Tooling](build-runtime-tooling.md)

## Risks And Gaps

- No rate limiting exists for repeated valid requests.
- User/session state is in-memory only, with no logout or token expiry.
- CORS/origin policy is permissive and not environment-scoped.
- Message contract is still implicit and unversioned.
- There is no persisted chat state or message history.

## Recommended Actions

1. Add session expiry/logout semantics and narrow allowed origins by environment.
2. Add rate limiting and contract-level tests.
3. Define a versioned message schema.
4. Add websocket lifecycle tests and restart/disconnect regression coverage.
5. Introduce structured logging and metrics hooks.

## Open Questions

- Should user/session state remain ephemeral for the project scope, or should accounts survive backend restarts?
- Should the API remain single-room, or should room/channel semantics be added?
- Should short-term in-memory history be exposed to reconnecting clients?