# Architecture Wiki Change Log

## 2026-04-23 (update 21)

- Removed stale references to the deleted `compose.yaml` path and the no-longer-supported direct local backend runtime from the architecture wiki.
- Re-synced system, component, drift, and risk docs to the actual runtime surface: DynamoDB Local, `sam local start-api`, `backend/src/aws_lambda.rs`, and `backend/src/bin/local_gateway.rs`.
- Updated the architecture scorecards and action lists so remaining work now focuses on deployed AWS validation, serverless fan-out limits, and operational hardening rather than unfinished local runtime migration.

## 2026-04-23 (update 20)

- Synced the wiki to the now-working SAM-local validation path: `backend/tests/aws_local_smoke.rs`, `make sam-local-smoke`, and the browser-verified Vite-to-SAM auth/websocket flow.
- Updated the AWS serverless and top-level architecture docs to remove stale assumptions about the old direct local backend path and to describe the local websocket gateway plus local DynamoDB-backed handler defaults.
- Re-ranked next steps toward deployed AWS validation, broader automation, and operational hardening now that local SAM auth and websocket smoke coverage exists.

## 2026-04-23 (update 19)

- Synced the wiki to the DynamoDB-backed Lambda implementation now living in `backend/src/aws_lambda.rs`, including persisted auth/session/connection state and API Gateway Management API fan-out.
- Updated the AWS docs and architecture pages to reflect the preserved frontend websocket contract via the `$default` route and the new local SAM workflow assets.
- Re-ranked next steps, risks, and drift to focus on SAM validation, deploy automation, observability, and production smoke testing rather than missing Lambda persistence scaffolding.

## 2026-04-23 (update 18)

- Collapsed the separate `backend-lambda/` crate into the main `backend/` crate so local Axum and AWS Lambda entry points now share one backend codebase.
- Updated the SAM references, implementation targets, and deployability notes across the wiki to point at the unified `backend/` crate.

## 2026-04-23 (update 17)

- Synced the wiki to the new AWS implementation scaffold: `infra/aws/template.yaml`, `backend-lambda/`, and the explicit frontend `VITE_AUTH_BASE_URL` contract.
- Updated next steps, risks, drift, and system overview so the AWS target is described as partially scaffolded rather than documentation-only.
- Re-ranked AWS follow-up work around DynamoDB-backed handlers, API Gateway Management API fan-out, and CI/SAM build automation.

## 2026-04-23 (update 16)

- Switched the documented production target from container-first to AWS serverless and added ADR-012 for Lambda + API Gateway + DynamoDB.
- Added `infra/aws/README.md` plus an `AWS Serverless Platform` component page to document the migration target and its constraints.
- Updated risks, drift, next steps, README, and system overview to reflect that the current Axum runtime is not directly deployable to Lambda.

## 2026-04-23 (update 15)

- Replaced the documented backend runtime from Python/FastAPI to Rust/Axum and updated component/detail docs to point at `backend/src/lib.rs` and Cargo-based tooling.
- Added ADR-011 to record the backend runtime migration and marked ADR-002 as superseded.
- Updated README local run and automated check instructions to use Cargo rather than Python tooling.

## 2026-04-23 (update 14)

- Synced the wiki to the hardened auth/session lifecycle: fixed session expiry, `POST /auth/logout`, configured origin restrictions, and backend auth lifecycle tests.
- Updated risk, drift, and next-step entries to remove the now-completed lifecycle hardening task and narrow the remaining auth gaps to persistence, refresh/rotation, and broader integration coverage.
- Updated the README and data-flow docs to describe `SESSION_TTL_SECONDS`, `ALLOWED_ORIGINS`, logout, and the backend test command.

## 2026-04-23 (update 13)

- Added baseline container assets: `backend/Dockerfile`, `frontend/Dockerfile`, `frontend/nginx.conf`, per-app `.dockerignore` files, and `compose.yaml`.
- Updated deployability, risks, and next steps to reflect that local containerized execution now exists while CI, production manifests, and runtime hardening remain open.
- Added README instructions for the containerized run path and documented the frontend build-time websocket contract used by compose.

## 2026-04-23 (update 12)

- Recorded container-first as the accepted production deployment target and removed the stale VM-vs-container decision point.
- Updated deployability language across the wiki and README so packaging gaps are described relative to the chosen container-first path.

## 2026-04-23 (update 11)

- Synced the wiki to the new auth/session implementation: `POST /auth/register`, `POST /auth/login`, token-gated websocket access, and server-owned sender identity.
- Updated the README and data-flow/system docs to describe the deployment-time contract that the configured websocket URL must align with the matching `/auth/*` endpoints.
- Re-ranked risks and next steps to mark impersonation mitigated and focus follow-up work on session lifecycle hardening, tests, reconnect UX, and deployment config.

## 2026-04-23 (update 10)

- Externalized the frontend websocket endpoint through `VITE_CHAT_WS_URL` in `frontend/src/App.jsx` and added `frontend/.env.example` as the documented local contract.
- Updated top-level README run instructions with the deployment-time socket configuration contract and examples.
- Re-ranked architecture priorities and mitigations to reflect that frontend endpoint configuration is now implemented while backend/deployment config conventions remain open.

## 2026-04-23 (update 9)

- Migrated the architecture wiki from `component-actions/` to `component-details/` so there is one dedicated detail file per identified component.
- Reworked [components](components.md) into a high-level component relationship/index page with concise summaries and links to the per-component detail files.
- Updated [README](README.md) and [next-steps](next-steps.md) so live navigation points to `component-details/` rather than the superseded action-file structure.

## 2026-04-23 (update 8)

- Added [Next Steps](next-steps.md) as the dedicated top-level action index for architecture work.
- Added `arch/component-actions/` with one action file per identified component: frontend UI, frontend styling, chat API, payload validator, connection manager, and build/runtime tooling.
- Updated [README](README.md) and [components](components.md) to link into the new action index and per-component action files.
- Added cross-links between component action files and related architecture docs to improve wiki navigation and reduce stale embedded recommendation lists.

## 2026-04-23 (update 7)

- Expanded the architecture NFR assessment to cover the additional qualities required by `.github/copilot-instructions.md`: robustness, modularity, reliability, fault tolerance, observability, testability, maintainability, privacy and data protection, usability, and accessibility.
- Added explicit rows for those qualities in `arch/system-overview.md` and aligned the high-level summaries in `arch/README.md`, `arch/risks.md`, and `arch/drift.md`.
- Added new risk `R-011` for missing accessibility validation and widened existing risk quality tags so the risk register maps to the full NFR set more accurately.
- Added a drift item for usability/accessibility gaps in the current chat UI behavior.

## 2026-04-23 (update 6)

- Reassessed NFR statuses against the current frontend and backend implementation rather than the desired target state.
- Downgraded Resilience from 🟢 good to 🟡 watch in the scorecards because backend cleanup is hardened but the frontend still has no reconnect/backoff behavior and there are no resilience tests.
- Added explicit client-reconnect gap to `arch/drift.md` and new risk `R-010` in `arch/risks.md`.
- Updated the high-level NFR summary in `arch/README.md` to match the revised scorecard posture.

## 2026-04-23 (update 5)

- Re-ran architecture sync checklist across all files in `arch/` against the current frontend and backend code.
- Verified NFR snapshots, risks, drift items, and prioritized work remain aligned after the resilience hardening changes.
- Refined `arch/data-flow.md` so the main chat sequence and disconnect flow explicitly match `_parse_and_validate()` and the `finally`-based cleanup behavior.

## 2026-04-23 (update 4)

- Re-synced embedded recommendation sections across `arch/` after completing resilience hardening.
- Updated `arch/README.md` NFR summary and reordered prioritized next work to remove the completed resilience item; added a recent-completions note.
- Corrected `arch/system-overview.md` NFR evidence so Availability/Resilience statuses and Security/Performance remediation reflect current payload validation and cleanup behavior.
- Removed obsolete duplicate proposed ADR from `arch/decisions.md`; ADR-007 now stands as the accepted cleanup decision.
- Removed completed resilience work from the active production-readiness path and aligned observability wording across `system-overview.md`, `risks.md`, and `drift.md` with the current basic-logging-only implementation.

## 2026-04-23 (update 3)

- Implemented guaranteed cleanup path for WebSocket handler — three-layer exception handling with nested try/except/finally to catch payload validation errors, `WebSocketDisconnect`, and broad runtime `Exception` types.
- Added ADR-007 documenting the try/except/finally cleanup pattern and its rationale.
- Marked R-002 (stale connections from non-disconnect exceptions) as mitigated.
- Updated drift log: resolved WebSocket reliability gap; moved Availability and Resilience from Weak to Good in NFR snapshot.
- Updated Chat API component to document guaranteed cleanup and full traceback logging; added logging to dependencies.
- Comprehensive test suite confirms handler resilience in all failure scenarios: disconnect, unexpected exception, and broadcast failure in finally block.

## 2026-04-23 (update 2)

- Documented protocol hardening work: new `_parse_and_validate()` component, updated data flow including validation error path and rules reference table, new Mermaid validation error sequence diagram.
- Marked R-003 (payload size/shape abuse) as mitigated with rate limiting gap noted.
- Added ADR-006 for payload validation approach.
- Updated drift log: malformed JSON path partially resolved; remaining gap is non-disconnect exception cleanup.
- Revised NFR hotspot and quality status snapshot to reflect input validation moving to 🟢 Good.

## 2026-04-23 (initial)

- Initialized architecture wiki files and navigation index.
- Documented current system boundaries and component responsibilities.
- Logged initial ADRs for transport, backend framework, and state strategy.
- Captured current architecture risks and one concrete drift item.
- Added assumptions for local development ports and runtime scope.
- Refreshed architecture docs against current React + FastAPI code in `frontend/` and `backend/`.
- Added explicit socket error-path data flow and health-check flow.
- Expanded risk register with reliability and testing gaps.
- Added temporary ADRs for local-first endpoint and unversioned protocol, plus proposed validation ADR.
- Updated drift log with concrete protocol/configuration mismatches and correction paths.
- Reworked architecture baseline to include explicit NFR and deployability assessment.
- Added NFR scorecard (Availability, Performance, Scalability, Security, Manageability, Flexibility, Portability, Cost, Resilience) with evidence and remediation.
- Expanded risk register and drift notes with deployment automation, observability, and portability gaps.
- Updated architecture priorities to include production deployment baseline work.
- Added traffic-light icons to NFR summaries and Mermaid diagrams for runtime topology and chat flow.
