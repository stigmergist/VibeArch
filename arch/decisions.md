# Decisions (ADR Lite)

## ADR-001: Use WebSocket for Chat Transport

- Status: accepted
- Date: 2026-04-23
- Decision: Implement chat communication over a single WebSocket endpoint (`/ws/chat`).
- Rationale: Real-time bi-directional messaging with minimal protocol overhead.
- Consequences: Must manage connection lifecycle and handle dropped sockets.

## ADR-002: Use FastAPI for Backend

- Status: superseded
- Date: 2026-04-23
- Decision: Build backend with FastAPI + Uvicorn.
- Rationale: Simple async WebSocket support and low setup complexity.
- Consequences: This was the initial implementation path and was later replaced by Rust + Axum.

## ADR-003: Keep Chat State In-Memory

- Status: accepted
- Date: 2026-04-23
- Decision: Do not persist messages, users, or session state in v1.
- Rationale: Keep project simple for learning/prototyping.
- Consequences: No history across refresh/restart; accounts and sessions reset on restart; no horizontal scaling safety.

## ADR-004: Hard-Code WebSocket Endpoint For Local-First v1

- Status: superseded
- Date: 2026-04-23
- Decision: Frontend uses a direct constant (`ws://localhost:8000/ws/chat`) in `frontend/src/App.jsx`.
- Rationale: Minimize configuration overhead during initial bring-up.
- Consequences: Environment portability was reduced until `VITE_CHAT_WS_URL` replaced the hard-coded constant later the same day.

## ADR-005: Keep Message Contract Unversioned In v1

- Status: accepted (temporary)
- Date: 2026-04-23
- Decision: Message envelope has no explicit schema version/type registry beyond current fields.
- Rationale: Move quickly with a minimal chat protocol.
- Consequences: Contract evolution risk increases as features are added.

## ADR-006: Validate Inbound WebSocket Payloads At Protocol Layer

- Status: accepted
- Date: 2026-04-23
- Decision: Implement `parse_and_validate()` in the shared backend module to enforce frame-size cap, JSON parse, object shape, field types, and length limits before any business logic.
- Rationale: Prevent malformed or oversized frames from crashing the socket handler or being broadcast; return structured error responses to the sender only rather than silently dropping or propagating.
- Consequences: Protocol constraints are now documented constants (`MAX_FRAME_BYTES`, `MAX_TEXT_CHARS`, `MAX_SENDER_CHARS`). Rate limiting is not yet covered and is a known remaining gap (see R-003).

## ADR-007: Guaranteed Cleanup with Try/Except/Finally Pattern

- Status: accepted
- Date: 2026-04-23
- Decision: Wrap the entire chat socket handler in a nested try/except/finally: inner guards payload validation, middle catches `WebSocketDisconnect`, outer catches broad `Exception`. Finally block always runs disconnect and leave-message broadcast, with failure-to-broadcast also caught and logged.
- Rationale: Ensure stale connections are never left in the manager registry, even under unexpected runtime errors. Structured logging of errors enables monitoring and post-incident analysis.
- Consequences: Handler is resilient to any exception type; connection cleanup overhead is minimal. Broadcast-failure logging may generate high volume under network instability (mitigated by mature connection cleanup patterns at the ConnectionManager level).

## ADR-008: Make Sender Identity Server-Owned After Auth

- Status: accepted
- Date: 2026-04-23
- Decision: Add `POST /auth/register` and `POST /auth/login` endpoints that issue fixed-lifetime session tokens, require `token` on `WS /ws/chat`, and reject any client payload that includes a `sender` field.
- Rationale: Prevent impersonation by moving identity ownership to the server while keeping the implementation simple enough for the current in-memory architecture.
- Consequences: Message payloads are simpler (`{ text }`), frontend deployment must keep websocket and auth endpoints aligned, and session lifecycle policy remains a separate concern from sender ownership.

## ADR-009: Treat Production Deployment As Container-First

- Status: superseded
- Date: 2026-04-23
- Decision: Standardize on a container-first production target for both frontend and backend rather than pursuing a VM-first deployment model.
- Rationale: Container packaging gives a repeatable runtime boundary, aligns with the documented managed-platform target, and reduces environment drift between development, CI, and deployment.
- Consequences: Dockerfiles and compose remain useful for local development, but the production target later changed to AWS serverless.

## ADR-010: Use Fixed-Lifetime Bearer Sessions With Explicit Logout And Origin Allowlist

- Status: accepted
- Date: 2026-04-23
- Decision: Keep auth lightweight by using in-memory bearer tokens with a fixed TTL, add `POST /auth/logout` for explicit revocation, and enforce an `ALLOWED_ORIGINS` allowlist for browser auth requests and WebSocket upgrades.
- Rationale: This closes the immediate impersonation/session-lifecycle gap without introducing persistence or a more complex token infrastructure before the project needs it.
- Consequences: Sessions now expire and can be revoked, browser origins are narrowed by configuration, and backend tests can validate the lifecycle. Sessions still reset on process restart and do not support refresh/rotation.

## ADR-011: Use Rust And Axum For The Backend Runtime

- Status: accepted
- Date: 2026-04-23
- Decision: Replace the Python FastAPI backend with a Rust Axum backend while preserving the existing auth and WebSocket contract exposed to the frontend.
- Rationale: The backend language/runtime should align with the desired Rust implementation without forcing a frontend protocol rewrite.
- Consequences: Backend build/test/container workflows now use Cargo, Rust integration tests cover auth lifecycle behavior, and the repository no longer depends on Python for the backend server implementation.

## ADR-012: Use AWS Serverless As The Production Deployment Target

- Status: accepted
- Date: 2026-04-23
- Decision: Use S3 + CloudFront for the frontend and API Gateway + Lambda + DynamoDB for backend behavior in production.
- Rationale: The desired billing model is pay-per-use rather than an always-on backend container, and AWS serverless infrastructure better matches that cost goal.
- Consequences: User, session, and connection state now live in DynamoDB on the AWS path, local development is aligned around SAM plus the websocket gateway shim, and the remaining work shifts to deploy automation, observability, and production validation.
