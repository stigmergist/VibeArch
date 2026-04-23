# Payload Validator

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Chat API](chat-api.md), [Connection Manager](connection-manager.md)

## Scope

Primary implementation: `parse_and_validate()` in `backend/src/lib.rs`

## Responsibilities

- Enforce payload size, shape, type, and length rules before chat messages enter the broadcast flow.
- Reject client attempts to supply server-owned fields such as `sender`.
- Normalize accepted data into the message structure expected by the chat handler.

## Dependencies

- `serde_json`
- [Chat API](chat-api.md)
- [Connection Manager](connection-manager.md)

## Risks And Gaps

- Validation rules are encoded in code constants but not exposed as a formal contract.
- There is no rate limiting layered on top of the size/shape checks.
- Coverage exists in shared backend unit tests and smoke tests, but there is still no dedicated contract test suite protecting all edge cases.

## Recommended Actions

1. Add automated tests for malformed payloads, length caps, and server-owned sender rejection.
2. Define/document a versioned schema or message contract.
3. Coordinate with [Chat API](chat-api.md) on rate limiting and error envelope stability.

## Open Questions

- Should blank messages remain silently discarded, or should clients receive a user-visible validation error?