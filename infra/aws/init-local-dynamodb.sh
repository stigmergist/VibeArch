#!/bin/sh

set -eu

ENDPOINT_URL="${DYNAMODB_ENDPOINT_URL:-http://localhost:8001}"
CONTENT_TYPE="Content-Type: application/x-amz-json-1.0"

call_dynamodb() {
  target="$1"
  payload="$2"

  curl -sS \
    -X POST "$ENDPOINT_URL" \
    -H "$CONTENT_TYPE" \
    -H "X-Amz-Target: DynamoDB_20120810.$target" \
    -d "$payload"
}

create_table_if_missing() {
  table_name="$1"
  attribute_name="$2"

  response="$(call_dynamodb DescribeTable "{\"TableName\":\"$table_name\"}" || true)"
  case "$response" in
    *'"Table"'* )
    echo "table already exists: $table_name"
    return 0
      ;;
  esac

  response="$(call_dynamodb CreateTable "{\"TableName\":\"$table_name\",\"AttributeDefinitions\":[{\"AttributeName\":\"$attribute_name\",\"AttributeType\":\"S\"}],\"KeySchema\":[{\"AttributeName\":\"$attribute_name\",\"KeyType\":\"HASH\"}],\"BillingMode\":\"PAY_PER_REQUEST\"}")"
  case "$response" in
    *'"TableDescription"'*|*'"ResourceInUseException"'* )
      echo "created table: $table_name"
      ;;
    * )
      echo "failed to create table: $table_name" >&2
      echo "$response" >&2
      exit 1
      ;;
  esac

  echo "created table: $table_name"
}

enable_ttl() {
  table_name="$1"
  attribute_name="$2"

  call_dynamodb UpdateTimeToLive "{\"TableName\":\"$table_name\",\"TimeToLiveSpecification\":{\"Enabled\":true,\"AttributeName\":\"$attribute_name\"}}" >/dev/null || true
}

create_table_if_missing simple-chat-users-local username
create_table_if_missing simple-chat-sessions-local token
create_table_if_missing simple-chat-connections-local connectionId

enable_ttl simple-chat-sessions-local expiresAtEpoch

echo "local DynamoDB tables are ready at $ENDPOINT_URL"