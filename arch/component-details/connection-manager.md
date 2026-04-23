# Connection Manager

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Chat API](chat-api.md), [Payload Validator](payload-validator.md), [Build And Runtime Tooling](build-runtime-tooling.md)

## Scope

Primary implementation: connection persistence and fan-out helpers in `backend/src/aws_lambda.rs` plus the local peer registry in `backend/src/bin/local_gateway.rs`

## Responsibilities

- Track active websocket connection identifiers.
- Fan out broadcasts to connected clients.
- Remove dead/stale connections during cleanup paths.
- Bridge API Gateway Management API semantics into the supported local websocket gateway.

## Dependencies

- DynamoDB
- API Gateway Management API
- Axum WebSocket support in the local gateway
- [Chat API](chat-api.md)
- [Build And Runtime Tooling](build-runtime-tooling.md)

## Risks And Gaps

- Broadcast is still sequential across stored connection records.
- The local gateway keeps a transient in-process peer map and is not itself horizontally scalable.
- There is no alternative fan-out adapter if the current scan-and-post model becomes too slow or costly.

## Recommended Actions

1. Validate the current scan-and-post fan-out model under expected load and cost envelopes.
2. Add tests covering dead-connection cleanup during broadcast.
3. Add observability around connection counts, stale connection cleanup, and broadcast failures.

## Open Questions

- Is DynamoDB-scan plus API Gateway post-to-connection sufficient for the expected scale path, or will room/channel support require a richer routing model?