# Architecture Wiki Change Log

## 2026-05-01 (update 39)

- Ran a lightweight architecture sync.
- Code-zone changes since update 38: `README.md` updated with a plain-language explanation of the ArchOps CoPilot concept (1 file, ~17 lines). No architectural boundaries, components, runtime behavior, or API contracts changed.
- Gate: change-volume gate not triggered (1 file changed, ~17 lines, no runtime surface affected).
- Supply chain recheck: Rust — 3 advisories in `rustls-webpki` (transitive via AWS SDK), consistent with prior scan; Node.js — 5 moderate advisories, consistent with prior scan. No new findings; R-016 evidence section remains current.
- Architecture wiki status: all docs confirmed current; no updates required to components, risks, drift, decisions, or next-steps.
- Re-ranked top 1-3 architecture actions remain:
  1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
  2. CI/CD + alarm routing + rollback baseline
  3. Explicit websocket-origin parity controls for the AWS path plus evidence-backed verification

## 2026-04-24 (update 38)

- Ran a gate-triggered architecture sync after extracting shared runtime policy code and parity tests across local and AWS paths.
- Updated `.arch/README.md`, `.arch/system-overview.md`, `.arch/data-flow.md`, `.arch/components.md`, `.arch/component-details/payload-validator.md`, `.arch/next-steps.md`, `.arch/risks.md`, and `.arch/drift.md` to reflect that payload validation and session TTL policy now share one implementation in `backend/src/runtime_contract.rs`.
- Re-scoped R-017 to the remaining websocket-origin parity gap and narrowed R-018 from duplicated policy logic to oversized orchestration modules.
- Scope reviewed: `backend/src/lib.rs`, `backend/src/aws_lambda.rs`, `backend/src/runtime_contract.rs`, `backend/tests/runtime_parity.rs`, and all updated `.arch/*.md` files listed above.
- Trigger signal: change-volume gate triggered by >=200 net changed lines outside `.arch/` (new shared module + parity test suite).
- Stable vs changed: message contract validation and session TTL policy parity are now stable across runtime paths; deployed AWS validation, websocket-origin parity controls, and CI/CD release enforcement remain open.
- Re-ranked top 1-3 architecture actions now are:
	1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
	2. CI/CD + alarm routing + rollback baseline
	3. Explicit websocket-origin parity controls for the AWS path plus evidence-backed verification

## 2026-04-24 (update 37)

- Applied new formatting standards from updated `.github/copilot-instructions.md` across all `.arch` files.
- Added opening plain-English summary paragraphs to `system-overview.md`, `risks.md`, `drift.md`, and `next-steps.md`.
- Reformatted the god-component anti-pattern finding in `risks.md` from dense prose into structured **What / Why it matters / Evidence / Resolution** blocks.
- Added a two-part Lightweight Threat Model section to `risks.md`: plain-English summary paragraph followed by a trust-boundary table with columns Trust Boundary | What crosses it | Current gap | Severity.
- No code-zone changes; this was a documentation formatting pass only.
- Gate: change-volume gate not triggered.


## 2026-04-24 (update 36)

- Ran a full architecture resync after the workspace-instruction changes added explicit requirements for threat modeling, anti-pattern tracking, preserved good patterns, and stricter architecture-sync validation.
- Updated `.arch/system-overview.md`, `.arch/risks.md`, `.arch/drift.md`, `.arch/data-flow.md`, `.arch/next-steps.md`, and `.arch/README.md` to reflect the real trust boundaries and current drift: the local runtime enforces `ALLOWED_ORIGINS` and configurable `SESSION_TTL_SECONDS`, while the AWS handler path still relies on token-only websocket connect checks and a default session TTL constant.
- Updated `.arch/decisions.md` to supersede the old in-memory-state ADR and add the supported DynamoDB-backed state-store decision for the AWS-oriented runtime.
- Added preserved good-pattern notes to the relevant component pages and recorded an emerging god-component anti-pattern risk backed by current file-size and responsibility concentration evidence.
- Revalidated dependency-audit evidence: `cargo audit` still reports 3 Rust advisories in `rustls-webpki`, `npm audit` still reports 5 moderate Node.js advisories, and the repo still has no project-managed Python dependency manifest.
- Scope reviewed: `.github/copilot-instructions.md`, `README.md`, `backend/src/lib.rs`, `backend/src/aws_lambda.rs`, `backend/tests/auth_lifecycle.rs`, `backend/Makefile`, `frontend/src/App.jsx`, `frontend/src/App.test.jsx`, `frontend/package.json`, `infra/aws/template.yaml`, and all `.arch/*.md` files.
- Gate result: change-volume gate not triggered because the code-zone runtime surface did not change in this pass; this was an explicit documentation resync driven by instruction changes and evidence cleanup.
- Re-ranked top 1-3 architecture actions now are:
	1. Deployed AWS validation plus local-to-AWS auth/origin policy parity
	2. CI/CD + alarm routing + rollback baseline
	3. Message-retention, privacy, and capacity guardrails for the persisted history path

## 2026-04-24 (update 35)

- Ran a full supply chain vulnerability audit for backend (Rust), frontend (Node.js), and Python dependencies.
- Findings: 3 moderate advisories in Rust (`rustls-webpki` via AWS SDK); 5 moderate advisories in Node.js (`vite`, `esbuild`, `vitest`); no actionable Python vulnerabilities.
- Added R-016 to `.arch/risks.md` and appended evidence section with details and remediation steps.
- Scope reviewed: `backend/Cargo.toml`, `backend/Cargo.lock`, `frontend/package.json`, `frontend/package-lock.json`, `.arch/risks.md`.
- Trigger signal: user request for full architecture rerun and vulnerability check; supply chain risk posture update.
- Stable vs changed: core architecture and boundaries unchanged; risk posture updated to reflect new supply chain evidence.
- Re-ranked top 1-3 architecture actions remain:
	1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
	2. CI/CD + alarm routing + rollback baseline
	3. Message-retention, privacy, and capacity guardrails for the new persisted history path

## 2026-04-24 (update 34)

- Migrated the architecture wiki folder from `arch/` to `.arch/` to improve discoverability, reduce accidental edits, and align with hidden knowledge-zone conventions.
- Updated all references in `.github/copilot-instructions.md`, project `README.md`, and `.arch/README.md` to use `.arch/` instead of `arch/`.
- Renamed the folder using `git mv` to preserve history and links.
- Validated that all navigation, scope, and cross-links in the architecture wiki now reference `.arch/` (except for historical entries in the change log and audit trails).
- Scope reviewed: `.github/copilot-instructions.md`, `README.md`, `.arch/README.md`, and all navigation/index files in `.arch/`.
- Trigger signal: user request for migration, 3+ core files changed, and knowledge-zone boundary update.
- Stable vs changed: all architecture knowledge and navigation is now in `.arch/`; no content or structure was lost, and all historical references remain for auditability.
- Re-ranked top 1-3 architecture actions remain:
	1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
	2. CI/CD + alarm routing + rollback baseline
	3. Message-retention, privacy, and capacity guardrails for the new persisted history path

## 2026-04-24 (update 33)

- Synced the wiki after adding persisted chat history and lazy backward pagination.
- Updated the system/runtime docs to reflect a new `Messages` persistence boundary, `GET /auth/messages` history replay, and the frontend behavior that loads recent history on join then fetches older pages only during upward scroll.
- Scope reviewed: `backend/src/lib.rs`, `backend/src/aws_lambda.rs`, `backend/src/bin/local_dynamodb_init.rs`, `backend/tests/auth_lifecycle.rs`, `frontend/src/App.jsx`, `frontend/src/App.test.jsx`, `infra/aws/template.yaml`, `infra/aws/env.local.json`, `README.md`, `infra/aws/README.md`, `arch/README.md`, `arch/system-overview.md`, `arch/data-flow.md`, `arch/next-steps.md`, `arch/risks.md`, `arch/drift.md`, `arch/component-details/chat-api.md`, `arch/component-details/frontend-ui.md`, and `arch/component-details/aws-serverless-platform.md`.
- Trigger signal: change-volume gate triggered (runtime persistence boundary changed, API contract expanded, and 10+ files changed overall).
- Stable vs changed: the single-room auth and websocket model remained stable; the data model expanded to include durable message history and cursor-based replay, which improved user continuity but added retention, privacy, and cost considerations.
- Re-ranked top 1-3 architecture actions now are:
	1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
	2. CI/CD + alarm routing + rollback baseline
	3. Message-retention, privacy, and capacity guardrails for the new persisted history path

## 2026-04-24 (update 32)

- Synced the wiki after adding CloudWatch monitoring resources to the AWS SAM stack.
- Updated deployability, risks, drift, and next-step posture to reflect that retained Lambda log groups, a service dashboard, and baseline alarms now exist, while alarm routing, threshold tuning, CI/CD, and repeated deployed smoke enforcement remain open.
- Scope reviewed: `infra/aws/template.yaml`, `infra/aws/README.md`, `README.md`, `arch/README.md`, `arch/system-overview.md`, `arch/next-steps.md`, `arch/risks.md`, `arch/drift.md`, and `arch/component-details/aws-serverless-platform.md`.
- Trigger signal: change-volume gate triggered (runtime infrastructure boundary changed in `infra/aws/template.yaml` and 8+ files changed overall).
- Stable vs changed: application behavior and deployment target remained stable; deployed monitoring maturity improved materially through a first-class CloudWatch dashboard and baseline alarms, while release automation and operational routing remain the top gaps.
- Re-ranked top 1-3 architecture actions now are:
	1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
	2. CI/CD + alarm routing + rollback baseline
	3. CI-enforced regression coverage that includes the deployed serverless flow

## 2026-04-24 (update 31)

- Synced the wiki after adding structured JSON backend logs and minimum service/SLO telemetry.
- Updated observability posture from "missing baseline" to "minimum baseline present" across the architecture docs, while keeping deployed alerting, dashboards, and release enforcement as open risks.
- Scope reviewed: `backend/src/telemetry.rs`, `backend/src/lib.rs`, `backend/src/aws_lambda.rs`, `backend/src/main.rs`, `backend/src/bin/local_gateway.rs`, `backend/src/bin/auth.rs`, `backend/src/bin/ws_connect.rs`, `backend/src/bin/ws_disconnect.rs`, `backend/src/bin/ws_message.rs`, `backend/tests/auth_lifecycle.rs`, `arch/README.md`, `arch/system-overview.md`, `arch/next-steps.md`, `arch/risks.md`, `arch/drift.md`, `arch/component-details/chat-api.md`, `arch/component-details/build-runtime-tooling.md`, and `arch/component-details/aws-serverless-platform.md`.
- Trigger signal: change-volume gate triggered (10+ code-zone files changed, including runtime observability behavior and a new shared telemetry module).
- Stable vs changed: system boundaries and deployment target remained stable; operational visibility improved materially through structured logs and health/SLO telemetry, while CI/CD, deployed alerting, and repeated AWS validation remain the top launch gaps.
- Re-ranked top 1-3 architecture actions remain:
	1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
	2. CI/CD + deployed alerting + rollback baseline
	3. CI-enforced regression coverage that includes the deployed serverless flow

## 2026-04-24 (update 30)

- Synced the wiki after adding broader backend and frontend regression coverage plus bounded frontend reconnect behavior.
- Updated architecture evidence to reflect the new Vitest frontend harness, expanded websocket lifecycle coverage in `backend/tests/auth_lifecycle.rs`, and the partial closure of reconnect/accessibility drift in `frontend/src/App.jsx`.
- Scope reviewed: `frontend/src/App.jsx`, `frontend/src/App.test.jsx`, `frontend/src/test/setup.js`, `frontend/package.json`, `frontend/vite.config.js`, `backend/tests/auth_lifecycle.rs`, `arch/README.md`, `arch/system-overview.md`, `arch/next-steps.md`, `arch/risks.md`, `arch/drift.md`, `arch/component-details/frontend-ui.md`, `arch/component-details/chat-api.md`, `arch/component-details/build-runtime-tooling.md`, and `arch/component-details/aws-serverless-platform.md`.
- Trigger signal: change-volume gate triggered (7 code-zone files changed, including runtime behavior and test-surface expansion).
- Stable vs changed: system boundaries and production target stayed stable; reconnect resilience, accessibility signaling, and regression evidence improved, while CI enforcement and deployed release validation remain the top gaps.
- Re-ranked top 1-3 architecture actions remain:
	1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
	2. CI/CD + observability + rollback baseline
	3. CI-enforced regression coverage that includes the deployed serverless flow

## 2026-04-24 (update 29)

- Added a deployed AWS smoke harness by reusing `backend/tests/aws_local_smoke.rs` for both SAM-local and deployed endpoints.
- Added `make aws-deployed-smoke`, which can consume explicit smoke endpoints or resolve `HttpApiUrl` and `WebSocketApiUrl` from CloudFormation outputs via `AWS_STACK_NAME` and `AWS_REGION`.
- Scope reviewed: `backend/tests/aws_local_smoke.rs`, `backend/Makefile`, `README.md`, `infra/aws/README.md`, `arch/README.md`, `arch/system-overview.md`, `arch/component-details/aws-serverless-platform.md`, `arch/component-details/build-runtime-tooling.md`, `arch/next-steps.md`, `arch/risks.md`, and `arch/drift.md`.
- Trigger signal: change-volume gate triggered (8 files changed, including test/runtime tooling and deployment validation workflow updates).
- Stable vs changed: production target and highest-risk themes remained stable; the release-validation path improved because deployed AWS smoke testing is now executable instead of only recommended.
- Re-ranked top 1-3 architecture actions remain:
	1. Deployed AWS path validation through repeated smoke runs and release checklist enforcement
	2. CI/CD + observability + rollback baseline
	3. Broader chat/reconnect integration and regression tests

## 2026-04-24 (update 28)

- Ran an explicit architecture sync after the local startup-hardening changes in `backend/Makefile`.
- Updated the wiki to reflect the new DynamoDB reachability and SAM build-artifact preflight checks, plus the refined assumption that `make local-aws-dev` is now a supported convenience launcher for the AWS-parity local backend path.
- Scope reviewed: `backend/Makefile`, `docker-compose.yml`, `backend/src/main.rs`, `README.md`, `arch/README.md`, `arch/system-overview.md`, `arch/next-steps.md`, `arch/drift.md`, and `arch/component-details/build-runtime-tooling.md`.
- Gate result: change-volume gate not triggered for this sync pass because the remaining architecture delta was documentation alignment around already-landed local tooling behavior.
- Stable vs changed: production target, top risks, and global architecture priorities stayed stable; local developer ergonomics and failure clarity improved.
- Re-ranked top 1-3 architecture actions remain:
	1. Deployed AWS path validation (smoke coverage beyond local SAM)
	2. CI/CD + observability + rollback baseline
	3. Broader chat/reconnect integration and regression tests

## 2026-04-24 (update 27)

- Added `docker-compose.yml` for one-command local bring-up (`docker compose up --build`) with frontend dev server + direct Axum backend container.
- Updated `backend/Dockerfile` to build and run the direct Axum binary in a compose-friendly local runtime image.
- Added `backend/src/main.rs` so the backend crate now exposes a runnable local binary (`simple-chat-backend`) for the compose profile.
- Updated local tooling/docs: new `make docker-local-up` and `make docker-local-down` in `backend/Makefile`, plus README quick-start guidance.
- Scope reviewed: `backend/src/main.rs`, `backend/Makefile`, `docker-compose.yml`, `README.md`, `arch/system-overview.md`, `arch/README.md`, `arch/drift.md`, and `arch/next-steps.md`.
- Trigger signal: change-volume gate triggered (8+ files changed and local runtime boundary expanded with a new compose path).
- Stable vs changed: production target and top deployment risks stayed stable; local onboarding/runtime ergonomics improved with a documented convenience profile.
- Re-ranked top 1-3 architecture actions remain:
	1. Deployed AWS path validation (smoke coverage beyond local SAM)
	2. CI/CD + observability + rollback baseline
	3. Broader chat/reconnect integration and regression tests

## 2026-04-24 (update 26)

- Resolved documentation drift for auth/session wording in `README.md` so it now reflects the supported DynamoDB-backed SAM/AWS session path instead of "in-memory session token" phrasing.
- Updated `arch/drift.md` to mark this wording mismatch as resolved while retaining the remaining auth lifecycle gaps (refresh/rotation strategy and Axum helper path parity).
- Gate result: change-volume gate not triggered (documentation-only update with no runtime, boundary, or contract change).

## 2026-04-24 (update 25)

- Ran an explicit architecture sync pass and revalidated `arch/next-steps.md`, `arch/risks.md`, and `arch/drift.md` against current code-zone evidence.
- Scope reviewed: `README.md`, `backend/src/*.rs`, `backend/src/bin/*.rs`, `backend/tests/*.rs`, `backend/Makefile`, `frontend/src/*`, `infra/aws/template.yaml`, and `infra/aws/README.md`.
- Gate result: change-volume gate not triggered for runtime/code-boundary changes in this pass (current unstaged change is instruction-language only in `.github/copilot-instructions.md`).
- Stable vs changed: architecture priorities remained stable; no runtime boundary, persistence model, API contract, or deployment-topology change detected since the prior sync.
- Re-ranked top 1-3 architecture actions remain:
	1. Deployed AWS path validation (smoke coverage beyond local SAM)
	2. CI/CD + observability + rollback baseline
	3. Broader chat/reconnect integration and regression tests
- Drift note still open: top-level `README.md` wording still says "in-memory session token" and should be aligned to the supported DynamoDB-backed SAM/AWS path.

## 2026-04-23 (update 24)

- Improved cross-doc scanability by adding explicit traffic-light "Scan First" cue blocks to `arch/README.md`, `arch/system-overview.md`, `arch/risks.md`, and `arch/drift.md`.
- Added visual priority legend and traffic-light section headers (`🔴`, `🟡`, `🟢`) in `arch/next-steps.md` so urgent work is identifiable at a glance.
- Kept technical recommendations unchanged; this update improves attention guidance and triage speed for readers.
- Gate result: change-volume gate not triggered by code-zone runtime changes in this pass; architecture update performed explicitly for documentation usability.

## 2026-04-23 (update 23)

- Reframed architecture outputs to be business and customer value first, with technical detail as supporting evidence, in `arch/next-steps.md`, `arch/risks.md`, and `arch/drift.md`.
- Added concise customer/business consequence summaries at the top of each of those docs so priorities are easier to communicate beyond engineering.
- Confirmed technical recommendations and prioritization remain unchanged; only framing and decision-readability were improved.
- Gate result: change-volume refresh gate not triggered by code-zone runtime changes in this pass; architecture sync was explicitly requested.

## 2026-04-23 (update 22)

- Ran an explicit architecture sync pass after the instruction-model update and evaluated the new change-volume gate.
- Gate result: not triggered for code-zone runtime changes in this pass (no boundary/API/persistence/runtime topology change detected; architecture content remained largely stable).
- Synced navigation consistency by adding the missing `component-details/aws-serverless-platform.md` link in `arch/next-steps.md`.
- Added a concrete documentation-drift note in `arch/drift.md` for the top-level `README.md` wording that still says "in-memory session token" despite the supported SAM-local/AWS path using DynamoDB-backed session state.
- Re-ranked top actions remained stable:
	1. Deployed AWS path validation (smoke coverage beyond local SAM)
	2. CI/CD + observability + rollback baseline
	3. Broader chat/reconnect integration and regression tests

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
