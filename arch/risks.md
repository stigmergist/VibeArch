# Architecture Risks

| ID | Risk | Quality Area | Severity | Likelihood | Mitigation | Owner |
|---|---|---|---|---|---|---|
| R-001 | No authentication allows impersonation (`sender` is client-provided) | Security | High | High | Add identity model and authenticated sessions/tokens; make sender server-owned post-auth | Backend owner |
| R-002 | Non-disconnect socket exceptions may skip immediate cleanup and leave stale connections | Availability, Resilience | High | Medium | Add broad exception handling with guaranteed disconnect/finally path and structured error logging | Backend owner |
| R-003 | Payload shape and size are validated, but per-connection/request rate limiting is still absent. | Security, Performance | Medium | Medium | Add per-connection rate limiting to complete mitigation | Backend owner |
| R-004 | In-memory connection store is single-process only | Scalability, Resilience | Medium | Medium | Add shared pub/sub adapter (for example Redis) before multi-instance deployment | Platform owner |
| R-005 | Frontend hard-codes backend URL to localhost | Portability, Deployability | Medium | High | Use environment-driven endpoint config (`VITE_CHAT_WS_URL`) | Frontend owner |
| R-006 | No automated tests for chat protocol or connection lifecycle | Manageability, Availability | Medium | High | Add backend websocket tests and frontend integration smoke tests | Full-stack owner |
| R-007 | No container/infra packaging for consistent runtime environments | Portability, Deployability | Medium | Medium | Add Dockerfiles and baseline deployment manifests/scripts | Platform owner |
| R-008 | No CI/CD workflow for build-test-release gatekeeping | Manageability, Resilience | Medium | High | Add CI workflow with lint/test/build and release checks | Platform owner |
| R-009 | No observability baseline (metrics/logging/alerting) | Availability, Performance | Medium | Medium | Add structured logs and minimum service/SLO telemetry | Backend owner |

## NFR Hotspots

- 🔴 Weak: Security (auth absent), Availability (no finally/cleanup), Resilience.
- 🟡 Watch: Performance (rate limiting absent), Scalability, Manageability, Portability, Cost.
- 🟢 Good: Flexibility, Input Validation (frame size, JSON shape, field limits hardened).
