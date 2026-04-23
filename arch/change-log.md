# Architecture Wiki Change Log

## 2026-04-23 (update 3)

- Synced wiki content against current backend/frontend code after protocol validation changes.
- Re-validated embedded "next steps" recommendations and removed stale "add protocol validation" guidance.
- Re-prioritized next work toward remaining resilience gap (non-disconnect exception cleanup), environment config externalization, and automated tests.
- Fixed decision log consistency by removing duplicate ADR numbering and replacing outdated proposed ADR with current proposed cleanup ADR.
- Updated risks and drift wording to reflect that payload validation is implemented while rate limiting and exception-safe cleanup remain open.

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
