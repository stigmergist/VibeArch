# Architecture Risks

| ID | Risk | Severity | Likelihood | Mitigation | Owner |
|---|---|---|---|---|---|
| R-001 | No authentication allows impersonation (`sender` is client-provided) | High | High | Add identity model and authenticated sessions/tokens; ignore client-supplied sender once authenticated | Backend owner |
| R-002 | Non-disconnect socket exceptions may skip immediate cleanup and leave stale connections | High | Medium | Add broad exception handling with guaranteed disconnect/finally path and structured error logging | Backend owner |
| R-003 | No payload size/rate limits can allow abuse and memory pressure | Medium | Medium | Add max message length and per-connection throttling | Backend owner |
| R-004 | In-memory connection store is single-process only | Medium | Medium | Add shared pub/sub adapter (for example Redis) before multi-instance deployment | Platform owner |
| R-005 | Frontend hard-codes backend URL to localhost | Medium | High | Use environment-driven endpoint config (`VITE_CHAT_WS_URL`) | Frontend owner |
| R-006 | No automated tests for chat protocol or connection lifecycle | Medium | High | Add backend websocket tests and frontend integration smoke tests | Full-stack owner |
