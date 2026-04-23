# Drift

## Quality Status Snapshot

- 🔴 Weak: observability.
- 🟡 Watch: availability, resilience, performance, scalability, security, manageability, portability, cost, robustness, reliability, fault tolerance, testability, maintainability, privacy and data protection, usability, accessibility.
- 🟢 Good: flexibility, input validation, modularity.

## Intended vs Observed

- Intended: runtime configuration should be environment-aware.
  - Observed (resolved 2026-04-23): frontend now reads `VITE_CHAT_WS_URL` in `frontend/src/App.jsx` and falls back to the local default when unset.
  - Remaining gap: backend runtime settings and deployment-time env injection conventions are still undocumented beyond the frontend socket contract and the UI's derived `/auth/*` expectation.
  - Status: 🟡 partially resolved.

- Intended: sender identity should be server-owned after authentication.
  - Observed (resolved 2026-04-23): `POST /auth/register` and `POST /auth/login` now mint in-memory session tokens, `WS /ws/chat` requires `?token=...`, and `backend/app/main.py` rejects any client-supplied `sender` while stamping outbound messages from the authenticated display name.
  - Remaining gap: user/session state is ephemeral, sessions have no logout or expiry, and CORS/origin policy remains permissive.
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
  - Observed: repository has no Dockerfile/compose files and no CI workflow definitions.
  - Impact: deployment is manual and environment drift risk is high.
  - Proposed correction: add container packaging and CI pipeline with build/test gates.

- Intended: operational behavior should be observable.
  - Observed: backend now emits basic application log messages for rejected payloads and unexpected runtime exceptions, but no structured logging, metrics, or alert hooks exist.
  - Impact: incident detection and diagnosis would be slow in production.
  - Proposed correction: introduce structured logs and minimal telemetry (latency, connection count, error rate).

- Intended: the chat experience should remain usable and accessible when messages arrive, validation errors occur, or connectivity changes.
  - Observed: the UI shows auth/connection state and validation errors, but it has no reconnect UX, no assistive-tech announcement path for incoming messages, and no documented accessibility verification.
  - Impact: users can be stranded after disconnects, and assistive-technology users may not be notified of new chat activity.
  - Proposed correction: add reconnect status UX, `aria-live` support for message updates, and an accessibility audit.

## Open Questions

- Should sessions expire after a fixed lifetime, on browser close, or only on explicit logout?
- Should reconnect include server-provided short history window?
- Is this service intended to stay single-room, or should room/channel concepts be introduced?
- Should deployment target be container-first (Kubernetes/managed container) or VM-first for v1 production?
