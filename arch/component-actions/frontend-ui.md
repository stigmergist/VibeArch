# Frontend UI Actions

## Navigation

- [Architecture Home](../README.md)
- [Next Steps](../next-steps.md)
- [Components](../components.md)
- Related components: [Frontend Styling](frontend-styling.md), [Chat API](chat-api.md), [Build And Runtime Tooling](build-runtime-tooling.md)

## Scope

Primary implementation: `frontend/src/App.jsx`

## Current Risks And Gaps

- Socket URL is hard-coded to `ws://localhost:8000/ws/chat`.
- There is no reconnect/backoff behavior after socket loss.
- Sender identity is still user-supplied.
- No accessibility support exists for announcing inbound messages to assistive technologies.

## Recommended Actions

1. Externalize socket configuration via `VITE_CHAT_WS_URL`.
2. Add reconnect/backoff behavior with clear user-facing retry state.
3. Add integration tests for connection lifecycle and error rendering.
4. Prepare the UI for authenticated/server-owned identity.
5. Add `aria-live` handling and keyboard-focused accessibility checks.

## Dependencies

- Depends on [Chat API](chat-api.md) for the WebSocket contract.
- Depends on [Frontend Styling](frontend-styling.md) for connection/error/reconnect presentation.
- Depends on [Build And Runtime Tooling](build-runtime-tooling.md) for environment injection and test automation.

## Open Questions

- Should reconnect automatically retry forever or stop after bounded attempts?
- Should the UI preserve unsent drafts across disconnect/reconnect?