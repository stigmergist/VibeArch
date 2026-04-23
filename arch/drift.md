# Drift

## Quality Status Snapshot

- 🔴 Weak: observability.
- 🟡 Watch: availability, resilience, performance, scalability, security, manageability, portability, cost, robustness, reliability, fault tolerance, testability, maintainability, privacy and data protection, usability, accessibility.
- 🟢 Good: flexibility, input validation, modularity.

## Intended vs Observed

- Intended: runtime configuration should be environment-aware.
  - Observed (resolved 2026-04-23): frontend now reads `VITE_CHAT_WS_URL` in `frontend/src/App.jsx` and falls back to the local default when unset.
  - Observed (extended 2026-04-23): backend now also reads `ALLOWED_ORIGINS` and `SESSION_TTL_SECONDS`, and `compose.yaml` injects those values for local container runs.
  - Remaining gap: deployment-time env injection conventions beyond the current frontend socket contract and backend auth/session settings are still incomplete.
  - Status: 🟡 partially resolved.

- Intended: sender identity should be server-owned after authentication.
  - Observed (resolved 2026-04-23): `POST /auth/register` and `POST /auth/login` now mint in-memory session tokens, `WS /ws/chat` requires `?token=...`, and `backend/app/main.py` rejects any client-supplied `sender` while stamping outbound messages from the authenticated display name.
  - Status: 🟢 fully resolved.

- Intended: sessions should be bounded, revocable, and restricted to configured browser origins.
  - Observed (resolved 2026-04-23): sessions now have a fixed TTL, `POST /auth/logout` revokes tokens, backend CORS is narrowed to `ALLOWED_ORIGINS`, and WebSocket upgrades reject disallowed `Origin` headers. `backend/tests/test_auth.py` covers expiry, logout revocation, and origin enforcement.
  - Remaining gap: sessions are still process-local, with no refresh/rotation strategy or persistence across restarts.
  - Status: 🟡 partially resolved.

- Intended: websocket handlers should fail safely and clean up reliably.
  - Observed (resolved 2026-04-23): Three-layer exception handling: inner try/except guards payload validation, middle try/except catches `WebSocketDisconnect` gracefully, outer try/except/finally catches broad `Exception` and guarantees cleanup. Disconnect is always called, and failure to broadcast leave message is logged but doesn't crash.
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
  - Observed: repository now has Dockerfiles and a local `docker compose` workflow, but it still has no CI workflow definitions or production deployment manifests.
  - Impact: local containerized execution is aligned with the architecture target, but production promotion and environment consistency remain underdefined.
  - Proposed correction: add CI pipeline, production deployment manifests, and runtime hardening on top of the compose baseline.

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
