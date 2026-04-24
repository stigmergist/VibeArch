# Chat API

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Payload Validator](payload-validator.md), [Connection Manager](connection-manager.md), [Frontend UI](frontend-ui.md), [Build And Runtime Tooling](build-runtime-tooling.md)

## Scope

Primary implementation: `backend/src/aws_lambda.rs`, `backend/src/lib.rs`, `backend/src/bin/auth.rs`, `backend/src/bin/ws_*.rs`

## Responsibilities

- Expose `GET /health` for basic health checks.
- Expose `POST /auth/register` and `POST /auth/login` for credential-based session creation.
- Expose `POST /auth/logout` for session revocation.
- Expose `GET /auth/messages` for cursor-based chat history retrieval.
- Expose `WS /ws/chat` for bidirectional chat transport.
- Coordinate session expiry, session lookup, origin checks, payload validation, message persistence, broadcast flow, lifecycle cleanup, server-side timestamps, and local/AWS route adaptation.

## Dependencies

- Axum
- Tokio runtime
- Rust crates for password hashing, token generation, JSON serialization, and CORS/WebSocket handling
- [Payload Validator](payload-validator.md)
- [Connection Manager](connection-manager.md)
- [Build And Runtime Tooling](build-runtime-tooling.md)

## Risks And Gaps

- No rate limiting exists for repeated valid requests.
- The supported path now persists user/session/message state, but the direct Axum helper path still keeps history only in memory.
- There is no refresh-token or token-rotation strategy.
- Message contract is still implicit and unversioned.
- Structured logs and minimum service counters now exist, but alarm routing and message-retention policy are still incomplete.

## Recommended Actions

1. Decide whether fixed-lifetime bearer sessions should gain refresh/rotation semantics.
2. Add rate limiting and move the growing contract-level tests into enforced CI coverage.
3. Define a versioned message schema.
4. Extend websocket and history-replay validation into deployed-flow regression coverage and release checks.
5. Define retention/privacy rules for persisted chat history and add alert thresholds plus capacity monitoring.

## Recent Evidence

- `backend/tests/auth_lifecycle.rs` now covers invalid client payload rejection without disconnecting the socket.
- `backend/tests/auth_lifecycle.rs` now also covers revoked-session behavior for an already-connected websocket client.
- `backend/src/lib.rs` and `backend/src/aws_lambda.rs` now emit structured auth, websocket, and broadcast log events, maintain minimum counters/SLO indicators through the shared telemetry module, and persist accepted chat messages for cursor-based history replay.

## Open Questions

- Should future auth add refresh tokens or token rotation, or keep forced re-login after expiry?
- Should the API remain single-room, or should room/channel semantics be added?
- Should stored history stay single-room and short-term, or should retention and room/channel semantics expand later?