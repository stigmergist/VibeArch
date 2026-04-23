# Payload Validator

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Chat API](chat-api.md), [Connection Manager](connection-manager.md)

## Scope

Primary implementation: `_parse_and_validate()` in `backend/app/main.py`

## Responsibilities

- Enforce payload size, shape, type, and length rules before chat messages enter the broadcast flow.
- Normalize accepted data into the message structure expected by the chat handler.

## Dependencies

- Python standard library (`json`)
- [Chat API](chat-api.md)
- [Connection Manager](connection-manager.md)

## Risks And Gaps

- Validation rules are encoded in code constants but not exposed as a formal contract.
- There is no rate limiting layered on top of the size/shape checks.
- No committed automated test suite currently protects edge cases.

## Recommended Actions

1. Add automated tests for malformed payloads, length caps, and sender normalization.
2. Define/document a versioned schema or message contract.
3. Coordinate with [Chat API](chat-api.md) on rate limiting and error envelope stability.

## Open Questions

- Should blank messages remain silently discarded, or should clients receive a user-visible validation error?