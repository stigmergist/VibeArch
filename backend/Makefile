ARTIFACTS_DIR ?= $(CURDIR)/.aws-sam/build
SAM ?= $(CURDIR)/../.sam-venv/bin/sam
SAM_TEMPLATE ?= $(CURDIR)/../infra/aws/template.yaml
SAM_BUILT_TEMPLATE ?= $(ARTIFACTS_DIR)/template.yaml
SAM_ENV_VARS ?= $(CURDIR)/../infra/aws/env.local.json
SAM_EVENTS_DIR ?= $(CURDIR)/../infra/aws/events
LOCAL_GATEWAY_PORT ?= 3001
CARGO_LAMBDA ?= $(HOME)/.cargo/bin/cargo-lambda

local-dynamodb-up:
	docker compose -f "$(CURDIR)/../compose.aws-local.yaml" up -d

local-dynamodb-down:
	docker compose -f "$(CURDIR)/../compose.aws-local.yaml" down

local-dynamodb-init:
	AWS_REGION="us-east-1" AWS_ACCESS_KEY_ID="local" AWS_SECRET_ACCESS_KEY="local" DYNAMODB_ENDPOINT="http://127.0.0.1:8001" cargo run --bin local_dynamodb_init

sam-build:
	"$(SAM)" build -t "$(SAM_TEMPLATE)"

sam-local-api:
	"$(SAM)" local start-api -t "$(SAM_BUILT_TEMPLATE)" --env-vars "$(SAM_ENV_VARS)"

sam-local-ws-gateway:
	WEBSOCKET_MANAGEMENT_ENDPOINT="http://127.0.0.1:$(LOCAL_GATEWAY_PORT)" \
	DYNAMODB_ENDPOINT="http://127.0.0.1:8001" \
	AWS_REGION="us-east-1" \
	AWS_ACCESS_KEY_ID="local" \
	AWS_SECRET_ACCESS_KEY="local" \
	cargo run --bin local_gateway

local-aws-up:
	@echo "Start these in separate terminals:"
	@echo "1. make local-dynamodb-up && make local-dynamodb-init"
	@echo "2. make sam-build && make sam-local-api"
	@echo "3. make sam-local-ws-gateway"

sam-local-ws-connect:
	"$(SAM)" local invoke WebSocketConnectFunction -t "$(SAM_BUILT_TEMPLATE)" --env-vars "$(SAM_ENV_VARS)" --event "$(SAM_EVENTS_DIR)/ws-connect.json"

sam-local-ws-message:
	"$(SAM)" local invoke WebSocketMessageFunction -t "$(SAM_BUILT_TEMPLATE)" --env-vars "$(SAM_ENV_VARS)" --event "$(SAM_EVENTS_DIR)/ws-message.json"

sam-local-ws-disconnect:
	"$(SAM)" local invoke WebSocketDisconnectFunction -t "$(SAM_BUILT_TEMPLATE)" --env-vars "$(SAM_ENV_VARS)" --event "$(SAM_EVENTS_DIR)/ws-disconnect.json"

sam-local-smoke:
	cargo test --test aws_local_smoke -- --ignored --nocapture

build-AuthFunction:
	"$(HOME)/.cargo/bin/cargo" +stable lambda build --release --bin auth --target aarch64-unknown-linux-gnu
	mkdir -p "$(ARTIFACTS_DIR)"
	cp "target/lambda/auth/bootstrap" "$(ARTIFACTS_DIR)/bootstrap"

build-WebSocketConnectFunction:
	"$(HOME)/.cargo/bin/cargo" +stable lambda build --release --bin ws_connect --target aarch64-unknown-linux-gnu
	mkdir -p "$(ARTIFACTS_DIR)"
	cp "target/lambda/ws_connect/bootstrap" "$(ARTIFACTS_DIR)/bootstrap"

build-WebSocketDisconnectFunction:
	"$(HOME)/.cargo/bin/cargo" +stable lambda build --release --bin ws_disconnect --target aarch64-unknown-linux-gnu
	mkdir -p "$(ARTIFACTS_DIR)"
	cp "target/lambda/ws_disconnect/bootstrap" "$(ARTIFACTS_DIR)/bootstrap"

build-WebSocketMessageFunction:
	"$(HOME)/.cargo/bin/cargo" +stable lambda build --release --bin ws_message --target aarch64-unknown-linux-gnu
	mkdir -p "$(ARTIFACTS_DIR)"
	cp "target/lambda/ws_message/bootstrap" "$(ARTIFACTS_DIR)/bootstrap"
