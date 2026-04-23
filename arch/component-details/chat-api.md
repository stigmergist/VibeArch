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
- Expose `WS /ws/chat` for bidirectional chat transport.
- Coordinate payload validation, broadcast flow, lifecycle cleanup, and server-side timestamps.

## Dependencies

- FastAPI
- Uvicorn runtime
- [Payload Validator](payload-validator.md)
- [Connection Manager](connection-manager.md)
- [Build And Runtime Tooling](build-runtime-tooling.md)

## Risks And Gaps

- No authentication or authorization boundary exists.
- No rate limiting exists for repeated valid requests.
- Message contract is still implicit and unversioned.
- There is no persisted chat state or message history.

## Recommended Actions

1. Add authenticated identity and server-owned sender handling.
2. Add rate limiting and contract-level tests.
3. Define a versioned message schema.
4. Add websocket lifecycle tests and restart/disconnect regression coverage.
5. Introduce structured logging and metrics hooks.

## Open Questions

- Should the API remain single-room, or should room/channel semantics be added?
- Should short-term in-memory history be exposed to reconnecting clients?