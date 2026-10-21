---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener presses Ctrl+Shift+B.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop
- text: The Product hides the Orca window and keeps the listening session going.
  kind: product
  entities:
  - entity: desktop-session
    effect: changes
    facts:
    - Window visibility
  actor: listener
  contexts:
    desktop:
      place: desktop
references:
- kind: code
  role: implementation
  target: app/src/platform/platform_integration.rs
- kind: code
  role: implementation
  target: app/src/platform/hotkey.rs
- kind: code
  role: implementation
  target: app/src/main.rs
---

# Hide Orca while music plays

## Trigger

The listener wants music to continue without the Orca window.

## Outcome

The window is hidden and music keeps playing; the tray and the shortcut bring Orca back.
