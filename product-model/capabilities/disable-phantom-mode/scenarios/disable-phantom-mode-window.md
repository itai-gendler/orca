---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Open Orca in the tray, clicks the tray icon or presses Ctrl+Shift+B.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop
- text: The Product shows and focuses the Orca window.
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

# Bring Orca back from the tray

## Trigger

Orca is hidden in Phantom Mode.

## Outcome

The window is shown again with the listening session intact.
