# Drift

## Quality Status Snapshot

- 🔴 Weak: observability.
- 🟡 Watch: availability, resilience, performance, scalability, security, manageability, portability, cost, robustness, reliability, fault tolerance, testability, maintainability, privacy and data protection, usability, accessibility.
- 🟢 Good: flexibility, input validation, modularity.

## Intended vs Observed

- Intended: runtime configuration should be environment-aware.
  - Observed (resolved 2026-04-23): frontend now reads `VITE_CHAT_WS_URL` in `frontend/src/App.jsx` and falls back to the local default when unset.
  - Observed (extended 2026-04-23): frontend now also supports explicit `VITE_AUTH_BASE_URL` override for AWS deployments where auth and websocket endpoints differ.
  - Observed (extended 2026-04-23): backend now also reads `ALLOWED_ORIGINS`, `SESSION_TTL_SECONDS`, AWS table names, and local AWS endpoint settings across the shared runtime.
  - Remaining gap: deployment-time env injection conventions beyond the current frontend socket/auth contract and backend auth/session settings are still incomplete.
  - Status: 🟡 partially resolved.

- Intended: sender identity should be server-owned after authentication.
  - Observed (resolved 2026-04-23): `POST /auth/register` and `POST /auth/login` now mint fixed-expiry session tokens, `WS /ws/chat` requires `?token=...`, and the shared backend path rejects any client-supplied `sender` while stamping outbound messages from the authenticated display name.
  - Status: 🟢 fully resolved.

- Intended: sessions should be bounded, revocable, and restricted to configured browser origins.
  - Observed (resolved 2026-04-23): sessions now have a fixed TTL, `POST /auth/logout` revokes tokens, backend CORS is narrowed to `ALLOWED_ORIGINS`, and WebSocket upgrades reject disallowed `Origin` headers. `backend/tests/auth_lifecycle.rs` covers expiry, logout revocation, and origin enforcement.
  - Observed (extended 2026-04-23): the supported SAM-local and AWS handler path now persists sessions in DynamoDB or DynamoDB Local.
  - Observed (documentation drift 2026-04-23): top-level `README.md` still says auth returns an "in-memory session token" even though the supported runtime stores sessions in DynamoDB-backed state.
  - Remaining gap: there is still no refresh/rotation strategy, and the Axum helper path used in tests remains process-local.
  - Proposed correction: update `README.md` wording so session behavior matches the supported SAM-local/AWS path.
  - Status: 🟡 partially resolved.

- Intended: websocket handlers should fail safely and clean up reliably.
  - Observed (resolved 2026-04-23): shared handler code now deletes persisted connection records during disconnect handling, and the local gateway also clears transient peer senders after socket termination.
  - Status: 🟢 fully resolved.

- Intended: clients should recover gracefully from transient socket loss or backend restarts.
  - Observed: frontend sets `connected` false on `onclose`/`onerror`, but there is no reconnect/backoff logic in `frontend/src/App.jsx`.
  - Impact: transient server restarts or network blips end the chat session until the user refreshes manually.
  - Proposed correction: add reconnect/backoff policy with bounded retries and UX feedback for reconnect attempts.

- Intended: public message contract should have explicit evolution path.
  - Observed: protocol is implicit/unversioned in UI and backend logic.
  - Impact: future feature additions can break compatibility accidentally.
  - Proposed correction: define message envelope schema and versioning strategy in docs and code.

- Intended: production deployability should be repeatable and automated.
  - Observed: repository now has `infra/aws/template.yaml`, a working local SAM workflow, a local websocket gateway, and shared Lambda handlers inside `backend/`; the old direct local backend path is no longer the supported runtime, but production still lacks deploy automation, secrets handling, and real AWS deployment validation.
  - Impact: the supported local backend path now matches the AWS-oriented code path, but production rollout remains incomplete.
  - Proposed correction: standardize SAM build/deploy automation, add CI-backed validation, and run deployed smoke tests.

- Intended: production backend billing should align closely with real usage through AWS Lambda.
  - Observed: the shared backend crate now contains DynamoDB-backed Lambda auth/session/connection handlers and API Gateway Management API fan-out, and the supported local backend path now runs through SAM plus the local websocket gateway shim.
  - Impact: the pay-per-use production model is now implementable, but it still needs deployment validation and cost guardrails before being treated as production-ready.
  - Proposed correction: validate the deployed Lambda path and add cost monitoring/budgets.

- Intended: operational behavior should be observable.
  - Observed: backend now emits basic application log messages for rejected payloads and unexpected runtime exceptions, but no structured logging, metrics, or alert hooks exist.
  - Impact: incident detection and diagnosis would be slow in production.
  - Proposed correction: introduce structured logs and minimal telemetry (latency, connection count, error rate).

- Intended: the chat experience should remain usable and accessible when messages arrive, validation errors occur, or connectivity changes.
  - Observed: the UI shows auth/connection state and validation errors, but it has no reconnect UX, no assistive-tech announcement path for incoming messages, and no documented accessibility verification.
  - Impact: users can be stranded after disconnects, and assistive-technology users may not be notified of new chat activity.
  - Proposed correction: add reconnect status UX, `aria-live` support for message updates, and an accessibility audit.

## Open Questions

- Should the project eventually add token refresh/rotation, or keep the current fixed-lifetime re-login model?
- Should reconnect include server-provided short history window?
- Is this service intended to stay single-room, or should room/channel concepts be introduced?
