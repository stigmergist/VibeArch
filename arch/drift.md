# Drift

## Quality Status Snapshot

- 🔴 Weak: security (auth absent), availability (no finally path), resilience.
- 🟡 Watch: performance (rate limiting absent), scalability, manageability, portability, cost.
- 🟢 Good: flexibility, input validation.

## Intended vs Observed

- Intended: runtime configuration should be environment-aware.
  - Observed: frontend uses hard-coded `ws://localhost:8000/ws/chat` in `frontend/src/App.jsx`.
  - Impact: deployment flexibility is limited and environment switches require code edits.
  - Proposed correction: move socket URL to Vite env variable (`VITE_CHAT_WS_URL`).

- Intended: websocket handlers should fail safely and clean up reliably.
  - Observed (partially resolved 2026-04-23): `_parse_and_validate()` guards against malformed JSON and invalid shapes; validation errors are returned to sender only; the receive loop continues. `WebSocketDisconnect` is still the only caught exception for loop exit, so non-disconnect runtime failures may still skip cleanup.
  - Remaining gap: add a broad `except Exception` with a guaranteed `manager.disconnect()` / `finally` path.
  - Status: 🟡 partially resolved.

- Intended: public message contract should have explicit evolution path.
  - Observed: protocol is implicit/unversioned in UI and backend logic.
  - Impact: future feature additions can break compatibility accidentally.
  - Proposed correction: define message envelope schema and versioning strategy in docs and code.

- Intended: production deployability should be repeatable and automated.
  - Observed: repository has no Dockerfile/compose files and no CI workflow definitions.
  - Impact: deployment is manual and environment drift risk is high.
  - Proposed correction: add container packaging and CI pipeline with build/test gates.

- Intended: operational behavior should be observable.
  - Observed: no explicit structured logging, metrics, or alert hooks in backend.
  - Impact: incident detection and diagnosis would be slow in production.
  - Proposed correction: introduce structured logs and minimal telemetry (latency, connection count, error rate).

## Open Questions

- Should join/leave events include user identity (once auth exists) or remain anonymous?
- Should reconnect include server-provided short history window?
- Is this service intended to stay single-room, or should room/channel concepts be introduced?
- Should deployment target be container-first (Kubernetes/managed container) or VM-first for v1 production?
