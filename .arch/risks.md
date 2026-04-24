# Architecture Risks

This file records known architecture risks, NFR hotspots, anti-patterns, and supply chain vulnerability findings. The most important thing to know right now is that deployed AWS validation, CI/CD, and local-to-AWS auth policy parity (R-013, R-008, R-017) are the top customer-trust risks and should be resolved before a production rollout.


## Customer And Business Risk Summary

- Highest customer trust risk: shipping the AWS path without repeated deployed validation, CI gates, and consistent auth/session/origin policy can create visible reliability or security surprises that hurt confidence early.
- Highest delivery-speed risk: missing CI/CD and broad integration coverage increases regression risk and slows feature velocity due to manual verification and rework.
- Highest cost and operations risk: telemetry improved, but missing routed alarm actions, undefined message-retention/capacity guardrails, and duplicated runtime-policy logic can still increase incident recovery time and make change safety harder to predict.

## Scan First (Traffic Light)

- 🔴 Act now: R-013 (deployed AWS path not yet validated consistently), R-017 (remaining websocket-origin policy drift), and R-008 (CI/CD still missing) are the most direct customer-trust and incident-recovery risks.
- 🟡 Watch closely: R-006, R-009, R-010, R-015, and R-018 can still amplify regressions, support burden, or cost surprises because history replay, monitoring, and large runtime modules remain partially unresolved.
- 🟢 Stable base: R-001, R-002, R-005, and the session/validation parity portion of R-017 are now mitigated enough to reduce blind spots.

| ID    | Risk                                                                                                                                                                                                                                        | Quality Area                                                           | Severity | Likelihood | Mitigation                                                                                                                      | Owner            |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- | -------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| R-001 | ~~No authentication allows impersonation (`sender` is client-provided)~~ **Mitigated (2026-04-23)**                                                                                                                                         | Security, Privacy and Data Protection                                  | Low      | Low        | None — mitigation complete                                                                                                      | Backend owner    |
| R-002 | ~~Non-disconnect socket exceptions may skip immediate cleanup and leave stale connections~~ **Mitigated (2026-04-23)**                                                                                                                      | Availability, Resilience                                               | Low      | Low        | None — mitigation complete                                                                                                      | Backend owner    |
| R-003 | Payload shape and size are validated, but per-connection/request rate limiting is still absent.                                                                                                                                             | Security, Performance                                                  | Medium   | Medium     | Add per-connection rate limiting to complete mitigation                                                                         | Backend owner    |
| R-004 | Websocket fan-out still depends on sequential posts across stored connection records, and the local gateway keeps a transient in-process peer map for socket delivery                                                                       | Scalability, Resilience                                                | Medium   | Medium     | Validate serverless fan-out behavior under load and define the next scale adapter if scan-based delivery becomes insufficient   | Platform owner   |
| R-005 | ~~Frontend hard-codes backend URL to localhost~~ **Mitigated (2026-04-23)** — frontend now reads `VITE_CHAT_WS_URL`; remaining deployment risk is documenting and injecting env values consistently per environment.                        | Portability, Deployability                                             | Low      | Low        | Maintain `.env.example` and deployment documentation for environment injection                                                  | Frontend owner   |
| R-006 | Backend and frontend regression coverage improved, but the expanded chat protocol, reconnect, and lifecycle tests are still not enforced in CI or release automation, and deployed AWS flow coverage is still thin                          | Manageability, Availability, Testability, Reliability, Maintainability | Medium   | Medium     | Enforce frontend/backend suites in CI and extend release-time validation for the deployed AWS flow                              | Full-stack owner |
| R-007 | The shared AWS SAM/Lambda path now exists and is the supported local backend workflow, but deploy automation, secrets handling, and runtime hardening are still incomplete beyond `sam local`                                               | Portability, Deployability                                             | Medium   | Medium     | Finish AWS deployment automation, secrets handling, and runtime hardening                                                       | Platform owner   |
| R-008 | No CI/CD workflow for build-test-release gatekeeping                                                                                                                                                                                        | Manageability, Resilience                                              | Medium   | High       | Add CI workflow with lint/test/build and release checks                                                                         | Platform owner   |
| R-009 | The backend now has structured JSON logs, minimum health/SLO telemetry, a CloudWatch dashboard, and baseline alarms, but alarm actions, threshold tuning, and operator runbooks are still missing                                         | Availability, Performance, Observability                               | Medium   | Medium     | Attach alarms to the chosen incident-routing path, tune thresholds from traffic, and document the response playbook             | Backend owner    |
| R-010 | Client now has bounded reconnect retries after socket loss, but restart/disconnect recovery is not yet validated end to end against the supported AWS release path, and sessions still end after retry exhaustion                           | Resilience, Availability, Reliability, Fault Tolerance, Usability      | Medium   | Medium     | Test restart/disconnect recovery end to end and decide whether session/draft continuity should survive retry exhaustion         | Frontend owner   |
| R-011 | The chat UI now announces status and message updates via live regions, but there is still no explicit accessibility validation for keyboard/focus behavior or contrast                                                                      | Accessibility, Usability                                               | Medium   | Medium     | Add accessibility audit and keyboard/focus checks                                                                               | Frontend owner   |
| R-012 | Local Axum user and session state are still in-memory only; the AWS path persists them in DynamoDB, but there is still no refresh/rotation strategy and local restarts still reset dev state                                                | Security, Reliability, Privacy and Data Protection                     | Medium   | Medium     | Decide whether token refresh/rotation is required and whether local/dev parity needs persistent storage                         | Backend owner    |
| R-013 | The chosen AWS Lambda production target now has a real implementation in the shared crate, and a deployed smoke harness exists, but it has not yet been validated consistently through deployed smoke runs, CI, or production observability | Portability, Deployability, Scalability, Reliability                   | Medium   | Medium     | Run deployed smoke validation as part of release checks, then add CI checks and operational telemetry before production rollout | Platform owner   |
| R-014 | AWS API Gateway WebSocket pricing adds connection-minute cost, so the pay-per-use expectation can be misunderstood                                                                                                                          | Cost                                                                   | Medium   | Medium     | Model expected concurrent usage and set CloudWatch budgets/alarms before production rollout                                     | Platform owner   |
| R-015 | Recent conversation history now persists and replays into the UI, but message-retention policy, privacy rules, and read/write capacity guardrails for the `Messages` table are still undefined                                           | Privacy and Data Protection, Cost, Availability, Usability             | Medium   | Medium     | Define retention/privacy policy, validate history query load, and add table capacity/cost monitoring                           | Platform owner   |
| R-016 | Supply chain vulnerabilities detected in backend (Rust) and frontend (Node.js) dependencies. Rust: 3 moderate advisories in `rustls-webpki` (transitive via AWS SDK). Node.js: 5 moderate advisories in `vite`, `esbuild`, `vitest`. | Security, Reliability, Manageability                                   | Medium   | Medium     | Schedule dependency upgrades for backend and frontend; monitor for upstream fixes in AWS SDK and major package updates.         | Platform owner   |
| R-017 | Local and AWS policy parity is improved by shared validation and session TTL helpers, but websocket origin enforcement still differs: local checks in-handler while AWS currently relies on edge controls. | Security, Privacy and Data Protection, Reliability, Portability        | Medium   | Medium     | Keep shared policy helpers and parity tests, and add explicit API Gateway/WAF websocket-origin enforcement plus validation evidence. | Platform owner   |
| R-018 | Runtime responsibilities are still concentrating in large files (`backend/src/aws_lambda.rs`, `backend/src/lib.rs`, `frontend/src/App.jsx`), increasing change risk even after duplicated validation/session policy was extracted. | Maintainability, Testability, Reliability, Robustness                  | Medium   | Medium     | Continue modular decomposition of runtime/UI orchestration and keep parity tests for shared policy boundaries.                 | Full-stack owner |

## NFR Hotspots

- 🟡 Watch: Availability, Resilience, Performance, Scalability, Security, Manageability, Portability, Cost, Observability, Robustness, Reliability, Fault Tolerance, Testability, Maintainability, Privacy and Data Protection, Usability, Accessibility.
- 🟢 Good: Flexibility, Input Validation (frame size, JSON shape, field limits hardened), Modularity.

## Observed Anti-Patterns

**Emerging god components**
- **What:** `backend/src/aws_lambda.rs` (1,213 lines) and `frontend/src/App.jsx` (414 lines) each accumulate multiple unrelated responsibilities.
- **Why it matters:** Large mixed-responsibility modules make change risky — a bug fix in one concern can break another, and tests are harder to isolate.
- **Evidence:** `aws_lambda.rs` mixes auth HTTP, DynamoDB, websocket orchestration, history paging, and validation. `App.jsx` mixes auth UX, reconnect logic, pagination, and message rendering.
- **Resolution:** Extract shared service modules, policy helpers, and thinner UI/runtime slices before adding more protocol features. See R-018.

## Lightweight Threat Model

The system's main exposure is at the websocket boundary: on the AWS path, authentication is checked at connect time using a bearer token, but the application-level origin enforcement applied locally is not enforced in the AWS Lambda handler. An attacker with a valid token could connect from any origin. All other boundaries are adequately controlled for the current stage.

| Trust Boundary | What crosses it | Current gap | Severity |
|---|---|---|---|
| Browser → Auth API (HTTP) | Registration and login JSON bodies | Input validated; origin enforced locally | 🟢 Low |
| Browser → WebSocket (connect) | Bearer token in query string | AWS path: app-level origin not enforced | 🔴 High |
| WebSocket → message frame | Chat message JSON | Validated; rate limiting absent | 🟡 Medium |
| Lambda → DynamoDB | Session, message, connection reads/writes | No gap identified | 🟢 Low |
| Deployment config | Env vars and SAM template | TTL parity now shared; websocket-origin control still not explicit on AWS path | 🟡 Medium |

## Supply Chain Vulnerability Evidence (2026-04-24)

- Revalidated during this sync with `cargo audit` and `npm audit`; current findings remain unchanged.
- **Rust (backend):**
  - 3 moderate vulnerabilities in `rustls-webpki` (transitive via AWS SDK): reachable panic, name constraint issues ([RUSTSEC-2026-0104](https://rustsec.org/advisories/RUSTSEC-2026-0104), [RUSTSEC-2026-0098], [RUSTSEC-2026-0099]).
  - Solution: Upgrade `rustls-webpki` to >=0.103.13 (requires upstream AWS SDK update).
- **Node.js (frontend):**
  - 5 moderate vulnerabilities in `vite`, `esbuild`, `vitest`, and related packages.
  - Solution: Upgrade `vite` to 8.0.10+ and `vitest` to 4.1.5+ (major version bumps required).
- **Python:** No project-managed Python dependency manifest exists. A `pip_audit` run was not possible in the current environment.
