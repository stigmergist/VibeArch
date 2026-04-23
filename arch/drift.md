# Drift

## Intended vs Observed

- Intended: runtime configuration should be environment-aware.
  - Observed: frontend uses hard-coded `ws://localhost:8000/ws/chat` in `frontend/src/App.jsx`.
  - Impact: deployment flexibility is limited and environment switches require code edits.
  - Proposed correction: move socket URL to Vite env variable (`VITE_CHAT_WS_URL`).

- Intended: websocket handlers should fail safely and clean up reliably.
  - Observed: `chat_socket` only catches `WebSocketDisconnect`; malformed JSON can raise outside that path.
  - Impact: reliability issues under malformed input; stale connection references possible until later cleanup.
  - Proposed correction: add payload parse/validation guard and guaranteed disconnect path.

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
