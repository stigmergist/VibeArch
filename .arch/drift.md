# Drift

This file tracks gaps between the intended architecture and what the code currently does. The most important thing to know right now is that local-to-AWS auth and session policy drift (session TTL, origin enforcement) is the top unresolved mismatch — it means behaviour validated locally may not match what runs in AWS.


## Customer And Business Consequence Snapshot

- Reliability perception risk: transient disconnect recovery improved, and recent conversation now restores on join, but release confidence still depends on proving that reconnect, history replay, and session-revocation behavior stay correct outside local validation.
- Security and trust risk: the docs previously implied stronger local-to-AWS policy parity than the code currently provides for session TTL and websocket origin handling.
- Support and onboarding risk: the convenience local runtime and the AWS-parity local runtime now coexist, so their intended uses must stay clearly documented to avoid incorrect validation assumptions.

## Scan First (Traffic Light)

- 🔴 Act now: deployed AWS validation, local-to-AWS auth/origin policy drift, and retention-policy/alarm-routing gaps still separate intended production readiness from current evidence.
- 🟡 Watch closely: message contract evolution, deployment automation, duplicated runtime-policy logic, and full accessibility verification remain partially resolved and user-impacting.
- 🟢 Stable base: sender ownership, payload validation, safe cleanup behavior, persisted recent history, baseline monitoring, and local runtime preflight checks are aligned with intended architecture.

## Quality Status Snapshot

- 🟡 Watch: availability, resilience, performance, scalability, security, manageability, portability, cost, observability, robustness, reliability, fault tolerance, testability, maintainability, privacy and data protection, usability, accessibility.
- 🟢 Good: flexibility, input validation, modularity.

## Intended vs Observed

- Intended: runtime configuration should be environment-aware.
  - Observed (resolved 2026-04-23): frontend now reads `VITE_CHAT_WS_URL` in `frontend/src/App.jsx` and falls back to the local default when unset.
  - Observed (extended 2026-04-23): frontend now also supports explicit `VITE_AUTH_BASE_URL` override for AWS deployments where auth and websocket endpoints differ.
  - Observed (extended 2026-04-23): backend local/helper runtime reads `ALLOWED_ORIGINS`, `SESSION_TTL_SECONDS`, AWS table names, and local AWS endpoint settings.
  - Remaining gap: `backend/src/aws_lambda.rs` still mints sessions with `DEFAULT_SESSION_TTL_SECONDS` rather than shared `SESSION_TTL_SECONDS`, and deployment-time env injection conventions beyond the current frontend socket/auth contract remain incomplete.
  - Status: 🟡 partially resolved.

- Intended: trust-boundary controls should stay consistent between the supported local path and the deployed AWS path.
  - Observed: `backend/src/lib.rs` rejects disallowed HTTP origins and websocket origins using `ALLOWED_ORIGINS`, but `backend/src/aws_lambda.rs` currently accepts websocket connects based on bearer-token validity alone, while `infra/aws/template.yaml` constrains HTTP CORS rather than websocket origin policy.
  - Impact: deployed websocket protection depends more on infrastructure perimeter settings than on the same shared application rules exercised locally.
  - Proposed correction: centralize origin policy in shared helpers where possible, or explicitly document and test the API Gateway/WAF control that replaces it.
  - Status: 🟡 partially resolved.

- Intended: payload validation and session policy should have one shared implementation path.
  - Observed: `parse_and_validate()` exists in both `backend/src/lib.rs` and `backend/src/aws_lambda.rs`, and session TTL policy differs between those runtime paths.
  - Impact: future message-contract or auth-policy changes can land in one runtime and silently miss the other.
  - Proposed correction: extract shared validation/session helpers plus parity tests that exercise both local and AWS handlers.
  - Status: 🔴 unresolved.

- Intended: sender identity should be server-owned after authentication.
  - Observed (resolved 2026-04-23): `POST /auth/register` and `POST /auth/login` now mint fixed-expiry session tokens, `WS /ws/chat` requires `?token=...`, and the shared backend path rejects any client-supplied `sender` while stamping outbound messages from the authenticated display name.
  - Status: 🟢 fully resolved.

- Intended: sessions should be bounded, revocable, and restricted to configured browser origins.
  - Observed (resolved 2026-04-23): sessions now have a fixed TTL, `POST /auth/logout` revokes tokens, backend CORS is narrowed to `ALLOWED_ORIGINS`, and WebSocket upgrades reject disallowed `Origin` headers. `backend/tests/auth_lifecycle.rs` covers expiry, logout revocation, and origin enforcement.
  - Observed (extended 2026-04-23): the supported SAM-local and AWS handler path now persists sessions in DynamoDB or DynamoDB Local.
  - Observed (resolved 2026-04-24): top-level `README.md` wording now matches the supported DynamoDB-backed SAM/AWS session path.
  - Remaining gap: there is still no refresh/rotation strategy, and the Axum helper path used in tests remains process-local.
  - Status: 🟡 partially resolved.

- Intended: websocket handlers should fail safely and clean up reliably.
  - Observed (resolved 2026-04-23): shared handler code now deletes persisted connection records during disconnect handling, and the local gateway also clears transient peer senders after socket termination.
  - Status: 🟢 fully resolved.

- Intended: clients should recover gracefully from transient socket loss or backend restarts.
  - Observed (partially resolved 2026-04-24): `frontend/src/App.jsx` now retries socket connection up to three times with explicit reconnect status feedback, restores recent persisted messages on join, and fetches older pages only during backward scroll; `frontend/src/App.test.jsx` covers reconnect plus backward-pagination behavior.
  - Remaining gap: restart/disconnect recovery is still not validated against the deployed AWS path, and the UI still drops the session after retry exhaustion instead of preserving longer continuity.
  - Status: 🟡 partially resolved.

- Intended: public message contract should have explicit evolution path.
  - Observed: protocol is implicit/unversioned in UI and backend logic.
  - Impact: future feature additions can break compatibility accidentally.
  - Proposed correction: define message envelope schema and versioning strategy in docs and code.

- Intended: production deployability should be repeatable and automated.
  - Observed: repository now has `infra/aws/template.yaml`, a working local SAM workflow, a local websocket gateway, and shared Lambda handlers inside `backend/`; the old direct local backend path is no longer the supported runtime, but production still lacks deploy automation, secrets handling, and real AWS deployment validation.
  - Observed (extended 2026-04-24): `docker-compose.yml` now provides a one-command local profile (`docker compose up --build`) for onboarding speed using frontend dev server + direct Axum backend (`backend/src/main.rs`).
  - Observed (extended 2026-04-24): `backend/Makefile` now performs explicit preflight checks for local DynamoDB reachability and SAM build artifacts before launching the SAM-local API or websocket gateway.
  - Observed (extended 2026-04-24): `backend/tests/aws_local_smoke.rs` and `backend/Makefile` now provide a deployed smoke path that can resolve the stack's `HttpApiUrl` and `WebSocketApiUrl` outputs and run the same register-plus-websocket round trip against AWS.
  - Caveat: this compose profile is a convenience runtime and not the AWS-parity validation path.
  - Impact: the supported local backend path now matches the AWS-oriented code path, but production rollout remains incomplete.
  - Proposed correction: standardize SAM build/deploy automation, add CI-backed validation, and run deployed smoke tests.

- Intended: component boundaries should stay small enough that policy changes can be made once and verified safely.
  - Observed: `backend/src/aws_lambda.rs` is 1,213 lines, `backend/src/lib.rs` is 955 lines, and `frontend/src/App.jsx` is 414 lines, with auth, transport, history, validation, and UX orchestration increasingly concentrated in those files.
  - Impact: maintenance cost and regression risk rise because future changes are more likely to create runtime-specific drift or broad retest needs.
  - Proposed correction: extract shared policy modules and thinner UI/runtime slices before adding more protocol or deployment features.
  - Status: 🟡 partially resolved.

- Intended: production backend billing should align closely with real usage through AWS Lambda.
  - Observed: the shared backend crate now contains DynamoDB-backed Lambda auth/session/connection handlers and API Gateway Management API fan-out, and the supported local backend path now runs through SAM plus the local websocket gateway shim.
  - Impact: the pay-per-use production model is now implementable, but it still needs deployment validation and cost guardrails before being treated as production-ready.
  - Proposed correction: validate the deployed Lambda path and add cost monitoring/budgets.

- Intended: operational behavior should be observable.
  - Observed (partially resolved 2026-04-24): backend entrypoints now emit structured JSON logs for auth, websocket, and broadcast events, the local `GET /health` route exposes minimum service counters, success-rate indicators, and SLO target thresholds, and `infra/aws/template.yaml` now provisions retained Lambda log groups, a CloudWatch dashboard, and baseline alarms.
  - Remaining gap: alarm actions, threshold tuning, and response runbooks still do not exist.
  - Impact: production visibility is materially better, but incident notification and response would still rely too much on manual monitoring.
  - Status: 🟡 partially resolved.

- Intended: conversation continuity should survive a new join without dumping the full chat log every time.
  - Observed (partially resolved 2026-04-24): the supported SAM/AWS handler path now persists chat messages and exposes `GET /auth/messages` with cursor-based backward pagination, while `frontend/src/App.jsx` loads the newest page on join and requests older pages only when the user scrolls near the top.
  - Remaining gap: deployed load, retention policy, and privacy expectations for stored message history are not yet defined.
  - Impact: user continuity is materially better, but production cost and data-handling risk remain underspecified.
  - Status: 🟡 partially resolved.

- Intended: the chat experience should remain usable and accessible when messages arrive, validation errors occur, or connectivity changes.
  - Observed (partially resolved 2026-04-24): the UI now shows reconnect status during bounded retries and marks both status and message regions as `aria-live`, while validation and auth errors continue to surface inline.
  - Remaining gap: there is still no documented accessibility audit, keyboard/focus verification, draft persistence, or history recovery after reconnect.
  - Impact: transient failures are less disruptive, but longer outages and accessibility regressions could still escape into production.
  - Status: 🟡 partially resolved.

## Open Questions

- Should the project eventually add token refresh/rotation, or keep the current fixed-lifetime re-login model?
- Should AWS websocket origin policy be enforced in-handler, at API Gateway, or by an explicit WAF/custom-domain boundary?
- Should reconnect include server-provided short history window?
- Is this service intended to stay single-room, or should room/channel concepts be introduced?
