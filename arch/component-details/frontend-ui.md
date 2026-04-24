# Frontend UI

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Frontend Styling](frontend-styling.md), [Chat API](chat-api.md), [Build And Runtime Tooling](build-runtime-tooling.md)

## Scope

Primary implementation: `frontend/src/App.jsx`

## Responsibilities

- Collect registration/login credentials and call the backend auth endpoints.
- Allow the user to revoke the current session via logout.
- Open and maintain the browser WebSocket connection.
- Render connection status, message list, and message composer.
- Send user-authored messages to the backend as JSON.
- Display server-side validation errors and disable sending while disconnected.

## Dependencies

- Browser WebSocket API
- React state/effect hooks
- [Chat API](chat-api.md)
- [Frontend Styling](frontend-styling.md)
- [Build And Runtime Tooling](build-runtime-tooling.md)

## Risks And Gaps

- Socket URL is now environment-driven via `VITE_CHAT_WS_URL`, and the UI may derive `/auth/*` from that value when `VITE_AUTH_BASE_URL` is unset, so deployment-time value management must stay documented and consistent.
- Reconnect behavior now uses bounded retries with visible status feedback, but the UI still clears session state after retry exhaustion.
- Session state is not persisted across refresh or explicit logout.
- Session expiry is enforced by the backend, but the UI has no pre-expiry warning or refresh flow.
- Live-region announcements now exist for status and message updates, but keyboard/focus accessibility has not been verified.

## Recommended Actions

1. Decide whether reconnect should preserve session state or drafts beyond the current bounded retry window.
2. Add integration tests for auth flow, connection lifecycle, and error rendering in CI.
3. Decide whether session state should survive refresh and whether expiry should surface a clearer UX than forced re-login.
4. Add keyboard-focused accessibility checks and an accessibility audit.
5. Keep the `VITE_CHAT_WS_URL` and `VITE_AUTH_BASE_URL` contract aligned with deployment documentation and build tooling.

## Recent Evidence

- `frontend/src/App.jsx` now retries unexpected socket closes up to three times and surfaces reconnect progress to the user.
- `frontend/src/App.jsx` now marks status and message regions as live announcements for assistive technologies.
- `frontend/src/App.test.jsx` now covers reconnect behavior and outbound payload shape with Vitest and Testing Library.

## Open Questions

- Should reconnect automatically retry forever or stop after bounded attempts?
- Should the UI persist the issued session token across refreshes or treat each load as a fresh sign-in?
- Should the UI expose session expiry timing or simply require re-login on failure?
- Should the UI preserve unsent drafts across disconnect/reconnect?