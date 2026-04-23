# Architecture Wiki Change Log

## 2026-04-23

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
