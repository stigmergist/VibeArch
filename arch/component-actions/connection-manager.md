# Connection Manager Actions

## Navigation

- [Architecture Home](../README.md)
- [Next Steps](../next-steps.md)
- [Components](../components.md)
- Related components: [Chat API](chat-api.md), [Payload Validator](payload-validator.md), [Build And Runtime Tooling](build-runtime-tooling.md)

## Scope

Primary implementation: `ConnectionManager` in `backend/app/main.py`

## Current Risks And Gaps

- Connection state is process-local only.
- Broadcast is sequential and in-process.
- There is no shared adapter for multi-instance fan-out.

## Recommended Actions

1. Define a shared pub/sub or broker-backed fan-out path.
2. Add tests covering dead-connection cleanup during broadcast.
3. Add observability around connection counts and broadcast failures.

## Dependencies

- Depends on [Chat API](chat-api.md) for lifecycle entry/exit and error handling.
- Depends on [Build And Runtime Tooling](build-runtime-tooling.md) for production topology and observability rollout.

## Open Questions

- Is Redis pub/sub sufficient for the expected scale path, or will room/channel support require a richer message bus?