# Payload Validator

This component protects the chat protocol boundary by enforcing payload and session policy rules in one shared module used by both local and AWS runtime paths.

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Chat API](chat-api.md), [Connection Manager](connection-manager.md)

## Scope

Primary implementation: `backend/src/runtime_contract.rs` (`parse_and_validate_chat_text()` and `SessionPolicy`)

## Responsibilities

- Enforce payload size, shape, type, and length rules before chat messages enter the broadcast flow.
- Reject client attempts to supply server-owned fields such as `sender`.
- Normalize accepted data into the message structure expected by the chat handler.
- Provide a shared, environment-aware session TTL policy consumed by both local and AWS auth/session paths.

## Dependencies

- `serde_json`
- `chrono`
- [Chat API](chat-api.md)
- [Connection Manager](connection-manager.md)

## Risks And Gaps

- Validation rules are encoded in code constants but not exposed as a formal contract.
- Websocket origin policy still differs by runtime path: local enforces in-handler checks while AWS currently depends on edge controls.
- There is no rate limiting layered on top of the size/shape checks.
- Coverage now includes unit and runtime parity tests, but there is still no contract-versioning strategy.

## Recommended Actions

1. Keep parity tests in place and extend them to cover additional malformed and edge-case payloads as the contract evolves.
2. Define/document a versioned schema or message contract.
3. Coordinate with [Chat API](chat-api.md) on rate limiting and error-envelope stability.
4. Document and test the explicit AWS control that enforces websocket origin policy parity.

## Good Patterns To Preserve

- Fail-closed protocol boundary: malformed JSON, oversize frames, blank payload normalization, and client-supplied `sender` rejection happen before persistence or broadcast.
- Shared policy source of truth: payload validation and session TTL policy are now implemented once and consumed by local and AWS paths.

## Open Questions

- Should blank messages remain silently discarded, or should clients receive a user-visible validation error?