# Chat API Actions

## Navigation

- [Architecture Home](../README.md)
- [Next Steps](../next-steps.md)
- [Components](../components.md)
- Related components: [Payload Validator](payload-validator.md), [Connection Manager](connection-manager.md), [Frontend UI](frontend-ui.md), [Build And Runtime Tooling](build-runtime-tooling.md)

## Scope

Primary implementation: `backend/app/main.py`

## Current Risks And Gaps

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

## Dependencies

- Depends on [Payload Validator](payload-validator.md) for protocol hardening.
- Depends on [Connection Manager](connection-manager.md) for broadcast semantics and cleanup.
- Depends on [Build And Runtime Tooling](build-runtime-tooling.md) for deployment, CI, and observability rollout.

## Open Questions

- Should the API remain single-room, or should room/channel semantics be added?
- Should short-term in-memory history be exposed to reconnecting clients?