# System Overview

## Goals

- Provide a simple real-time chat experience for multiple browser clients.
- Keep implementation lightweight and easy to run locally.

## Boundaries

- Frontend runtime: browser, served by Vite dev server (`frontend/`).
- Backend runtime: FastAPI server (`backend/`).
- Persistence: none (in-memory only).
- Wire protocol: unversioned JSON messages over a single WebSocket endpoint.

## Runtime Context Narrative

- Users open the React app and connect over WebSocket to the backend.
- Backend tracks active socket connections in memory.
- Incoming client messages are broadcast to all active clients.
- Backend also emits system join/leave messages.
- Health check is exposed at `GET /health`.

## Runtime Topology

```mermaid
flowchart LR
	Browser[Browser Client\nReact + Vite UI] <-->|WebSocket JSON\n/ws/chat| API[FastAPI App\nbackend/app/main.py]
	Browser -->|HTTP asset requests| Vite[Vite Dev Server]
	API -->|In-memory state| Manager[ConnectionManager\nprocess-local connection list]
	Monitor[Operator or Probe] -->|GET /health| API
```

## Major Runtime Concerns

- Connection lifecycle management for disconnect/reconnect.
- Input validation enforced via `_parse_and_validate()` (frame size, JSON parse, shape, field types, length limits).
- Validation errors returned to sender only; no broadcast of rejected payloads.
- No authentication or authorization in current scope.
- No data persistence or chat history retention.
- Single-process memory model limits horizontal scalability.

## Assumptions

- Development environment uses `localhost` with frontend on `5173` and backend on `8000`.
- Frontend and backend are launched separately during local development.
- Message timestamps are generated server-side in UTC ISO-8601 format.

## NFR Scorecard

| Quality | Status | Evidence | Top Remediation |
|---|---|---|---|
| Availability | 🟡 watch | In-memory process-local state only; no data persistence; health check is static. | Add persistent message store and process supervision with graceful restart strategy. |
| Performance | 🟡 watch | Broadcast loop sends per-connection sequentially from Python process memory; frame/shape limits exist but no throughput profiling or rate limiting is present. | Add per-connection rate limiting and basic latency/throughput measurements before feature growth. |
| Scalability | 🟡 watch | `ConnectionManager` is process-local list; no shared state/pub-sub for multi-instance fan-out. | Introduce Redis pub/sub (or equivalent) for horizontal scale path. |
| Security | 🔴 weak | No auth; sender identity is client-supplied; payload validation exists but per-connection rate limiting and identity controls are absent. | Add authn/authz boundary and server-owned identity fields with rate limiting. |
| Manageability | 🟡 watch | No CI workflow, no operational runbook/deployment scripts, and only basic application logging. | Add CI checks, structured logs, and minimal operational runbook. |
| Flexibility | 🟢 good | Clean frontend/backend split and simple protocol permit iterative change. | Preserve separation while introducing schema/versioning and env config. |
| Portability | 🟡 watch | Frontend socket URL is now environment-driven via `VITE_CHAT_WS_URL`, but no container spec or deployment packaging exists yet. | Add Docker-based runtime packaging and document environment injection per deployment target. |
| Cost | 🟡 watch | Low current runtime footprint, but no cost controls/limits for future scaling. | Define deployment sizing defaults and autoscaling/capacity guardrails. |
| Resilience | 🟡 watch | Backend cleanup is exception-safe, but the client has no reconnect/backoff logic and there are no automated failure-injection tests for disconnect/restart scenarios. | Add client reconnect/backoff policy and backend/frontend resilience tests around restart and disconnect behavior. |
| Robustness | 🟡 watch | Invalid payloads are handled safely, but the wire contract is still implicit/unversioned and the app has no persisted recovery state. | Define a versioned message schema and add contract tests for malformed/edge-case inputs. |
| Modularity | 🟢 good | Frontend and backend are cleanly separated, and backend responsibilities are split across transport, validation, and connection-management code paths. | Preserve module boundaries while adding auth, config, and scaling adapters. |
| Reliability | 🟡 watch | Core chat flow works in a single process, but messages are lost on restart, clients do not reconnect automatically, and there is no automated regression suite in-repo. | Add reconnect behavior, persistence strategy, and automated websocket regression tests. |
| Fault Tolerance | 🟡 watch | The server tolerates malformed input and runtime exceptions within one process, but there is no redundancy, failover, or graceful degradation across process loss. | Add multi-instance/shared-state strategy and define restart/failover behavior. |
| Observability | 🔴 weak | Only basic logger calls exist for rejected payloads and unexpected exceptions; no structured logs, metrics, tracing, or alerting are present. | Add structured logging, connection/error metrics, and alerting hooks. |
| Testability | 🟡 watch | `_parse_and_validate()` and the socket handler are testable in isolation, but there is no committed automated backend/frontend test suite or CI execution. | Add backend websocket tests, frontend integration tests, and run them in CI. |
| Maintainability | 🟡 watch | The codebase is small and documented, but hard-coded runtime config, no CI, and missing automated tests increase change risk over time. | Externalize config and add automated validation around build, protocol, and deployment assumptions. |
| Privacy and Data Protection | 🟡 watch | No server-side persistence reduces retained user data, but there is no explicit privacy posture, TLS deployment requirement, or authenticated identity boundary for production use. | Define privacy/data handling expectations and require authenticated, TLS-protected deployments. |
| Usability | 🟡 watch | The UI exposes connection state, disables sending when disconnected, and shows server error messages, but there is no reconnect UX, history window, or delivery-state feedback. | Add reconnect UX/status messaging and basic session continuity behavior. |
| Accessibility | 🟡 watch | The UI uses native form controls and a visible label, but there is no `aria-live` support for new messages, no keyboard/accessibility audit, and color contrast has not been verified. | Add live-region announcements, keyboard/focus checks, and an accessibility review. |

## Deployability Assessment

### Where It Can Be Deployed Now

- Local developer machine: ready.
- Single VM/manual deployment: possible with manual process startup for frontend and backend.
- Containerized or managed platform deployment: not yet ready as no Dockerfile/compose or platform manifests are present.

### Missing For Production Deployment

- Configuration management for runtime endpoints beyond the documented frontend `VITE_CHAT_WS_URL` contract (for example backend settings and deployment injection).
- Secrets strategy (none defined yet).
- CI/CD pipeline and automated test gate (no workflow files detected).
- Observability baseline (structured logs, metrics, alerting).
- Rollback/release strategy and environment promotion model.
- Capacity planning and load profile for websocket fan-out behavior.

### Recommended Target And Smallest Path To Production

- Target model: containerized frontend and backend on a managed platform with TLS termination and external pub/sub for scale.
- Smallest path:
	1. Introduce backend settings model and deployment-time environment injection conventions around the documented frontend `VITE_CHAT_WS_URL` contract.
	2. Add Dockerfiles and a simple compose/dev deployment profile.
	3. Add CI pipeline for lint/test/build.
	4. Add structured logging and minimum health/readiness checks.
	5. Define release and rollback procedure for frontend/backend deployments.
