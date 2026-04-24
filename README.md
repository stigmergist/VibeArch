# Demonstrator Application for ArchOps CoPilot idea

The copilot-instructions.md in /.github is the main thing of interest.

The commit history shows how this influences architectural improvement

## Simple Chat App (React + Rust)

A minimal real-time chat application with:
- React frontend (Vite)
- Rust backend handlers that run through AWS Lambda/SAM locally and in AWS

## Project structure

- `frontend/` React chat UI
- `backend/` shared Rust backend crate for the Lambda auth and websocket handlers
- `.arch/` (hidden) architecture wiki and decision log

## Run locally

The recommended AWS-parity local backend path is the SAM-targeted local stack.

### Docker Compose (single command)

For a fast local run in one command:

```bash
docker compose up --build
```

Then open `http://localhost:5173`.

Stop it with:

```bash
docker compose down
```

Notes:
- This compose profile runs the frontend dev server plus a direct Axum backend container (`backend/src/main.rs`) on `http://localhost:8000`.
- This is intended for quick local usage and onboarding convenience.
- The SAM-local + websocket-gateway path below remains the AWS-parity validation path.

### 1. Install local prerequisites

```bash
python3 -m venv .sam-venv
./.sam-venv/bin/pip install --upgrade pip
./.sam-venv/bin/pip install aws-sam-cli
cargo install cargo-lambda
```

### 2. Start local DynamoDB

```bash
cd backend
make local-dynamodb-up
make local-dynamodb-init
```

### Fast Path (repeat runs)

Backend in one terminal:

```bash
cd backend
make local-aws-dev
```

Frontend in a second terminal:

```bash
cd frontend
npm run dev
```

To stop local DynamoDB when done:

```bash
cd backend
make local-aws-down
```

### 3. Build and start the local AWS stack

Terminal 1:

```bash
cd backend
make sam-build
make sam-local-api
```

Terminal 2:

```bash
cd backend
make sam-local-ws-gateway
```

Local endpoints:
- Auth API: `http://127.0.0.1:3000/auth/*`
- Chat socket: `ws://127.0.0.1:3001/ws/chat`

### 3a. Run the local smoke test

With DynamoDB Local, `sam local start-api`, and the local websocket gateway running:

```bash
cd backend
make sam-local-smoke
```

This registers a fresh user against the local SAM auth API, connects to the local websocket gateway with the returned token, sends a chat message, and asserts the echoed chat envelope.

### 3b. Run the deployed AWS smoke test

After deploying the SAM stack to AWS, run the same register-connect-send-receive flow against the deployed handlers:

```bash
cd backend
AWS_STACK_NAME=<your-stack-name> AWS_REGION=<your-region> make aws-deployed-smoke
```

This resolves `HttpApiUrl` and `WebSocketApiUrl` from the CloudFormation stack outputs and runs the deployed smoke case against `/auth/register` plus the `$default` websocket route.

If you already know the endpoints, you can also provide them directly:

```bash
cd backend
SMOKE_AUTH_BASE_URL=https://.../auth SMOKE_CHAT_WS_URL=wss://.../prod make aws-deployed-smoke
```

### 4. Start frontend (new terminal)

```bash
cd frontend
npm install
cp .env.example .env
npm run dev
```

Open `http://localhost:5173`.

## AWS Deployment Direction

The repo now treats the AWS-targeted handler code as the only supported backend path for local development and deployment.

For an AWS pay-per-use deployment, the backend needs to move to AWS-native serverless primitives:
- S3 + CloudFront for the frontend
- API Gateway HTTP API + Lambda for `POST /auth/register`, `POST /auth/login`, `POST /auth/logout`, and `GET /auth/messages`
- API Gateway WebSocket API + Lambda for `$connect`, `$disconnect`, and chat message routes
- DynamoDB for users, sessions, persisted chat messages, and active WebSocket connection records

The implementation target and migration notes live in `infra/aws/README.md`.

Frontend environment contract:
- `VITE_CHAT_WS_URL`: full websocket URL used by the browser client.
- Local default/example: `ws://127.0.0.1:3001/ws/chat`
- Production example: `wss://chat.example.com/ws/chat`
- `VITE_AUTH_BASE_URL`: optional explicit auth API base URL.
- Local default/example: `http://127.0.0.1:3000/auth`
- If `VITE_AUTH_BASE_URL` is unset, the frontend derives the auth API base URL from `VITE_CHAT_WS_URL` by switching `ws` -> `http` and replacing the trailing `/ws/chat` with `/auth`.
- Deployment contract: set `VITE_AUTH_BASE_URL` when the HTTP auth API and WebSocket API do not share the same public base URL.

## AWS Scaffold

The repository now includes the first AWS serverless scaffold:
- `infra/aws/template.yaml`: AWS SAM template for S3, DynamoDB, HTTP API, WebSocket API, Lambda wiring, retained Lambda log groups, a CloudWatch dashboard, and baseline CloudWatch alarms.
- `backend/`: the single Rust backend crate now contains the shared AWS handlers, Lambda binaries, a local DynamoDB bootstrap utility, and a small local websocket gateway that emulates the API Gateway websocket surface for development.

Current implementation status:
- There is now one backend codebase rather than separate local and AWS backend crates.
- Shared validation, password hashing, session persistence, connection persistence, and websocket fan-out live in the main backend crate.
- Local auth runs through `sam local start-api`.
- Local websocket chat runs through the same shared AWS handler logic behind the local websocket gateway.

Build prerequisite:
- `backend/Makefile` expects `cargo-lambda` to be installed for Linux `bootstrap` builds.
- `make aws-deployed-smoke` uses the `aws` CLI when stack outputs are resolved automatically from `AWS_STACK_NAME` and `AWS_REGION`.
- the deployed stack now also outputs `CloudWatchDashboardName` for the provisioned service dashboard.

Local/AWS backend shape:
- Local auth API: `sam local start-api`
- Local websocket gateway: `cargo run --bin local_gateway`
- AWS Lambda binaries from the same crate: `auth`, `ws_connect`, `ws_disconnect`, `ws_message`
- SAM now builds from `backend/`, so the code you deploy to AWS lives in the same crate you run locally.

## How it works

- User creates an account or signs in from the frontend; the backend returns a fixed-expiry session token backed by DynamoDB or DynamoDB Local in the supported SAM/AWS handler path.
- Frontend opens a WebSocket connection to `VITE_CHAT_WS_URL?token=...` and falls back to `ws://127.0.0.1:3001/ws/chat` when the env var is unset.
- Backend only accepts HTTP auth requests and websocket connections through the AWS-local path.
- Client sends `{ text }` payloads only.
- Backend authenticates the socket from the session token and stamps `sender` from the authenticated identity.
- Frontend can sign out by calling `POST /auth/logout` with `Authorization: Bearer <token>`.
- Frontend loads the newest saved messages on join, then requests older pages only when the user scrolls back toward the top of the message list.
- Backend rejects any client payload that tries to send its own `sender` field.
- Backend persists each valid chat message, then broadcasts it to all connected clients.
- Join/leave events are sent as system messages.
- AWS Lambda deployment and local SAM development both use the shared DynamoDB-backed handler module.

## Automated Checks

Backend auth/session lifecycle coverage:

```bash
cd backend
cargo test
```
