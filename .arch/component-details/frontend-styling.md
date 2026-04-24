# Frontend Styling

## Navigation

- [Architecture Home](../README.md)
- [Components](../components.md)
- [Next Steps](../next-steps.md)
- Related components: [Frontend UI](frontend-ui.md)

## Scope

Primary implementation: `frontend/src/styles.css`

## Responsibilities

- Define the visual theme and responsive layout.
- Provide message, status, and error-state presentation for the chat UI.

## Dependencies

- Native CSS only
- [Frontend UI](frontend-ui.md)

## Risks And Gaps

- Accessibility has not been verified for color contrast, focus states, or screen-reader behavior.
- There is no styling yet for reconnect states or delivery-progress feedback.

## Recommended Actions

1. Audit color contrast and focus visibility.
2. Add styles for reconnecting/retry UX once [Frontend UI](frontend-ui.md) implements it.
3. Review message-list semantics alongside accessibility improvements.

## Open Questions

- Does the project want a formal accessibility target such as WCAG 2.1 AA?