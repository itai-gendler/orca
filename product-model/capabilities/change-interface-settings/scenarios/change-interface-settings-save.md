---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener picks the sidebar mode or the seekbar style, or shows or hides likes.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::interface
- text: The Product saves preferences for the profile.
  kind: product
  entities:
  - entity: preferences
    effect: changes
    facts:
    - Interface
    - Show Liked Songs
  actor: listener
  contexts:
    desktop:
      place: desktop::interface
references:
- kind: code
  role: implementation
  target: app/ui/settings.slint
- kind: code
  role: implementation
  target: app/src/controllers/preferences.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
---

# Change Interface settings

## Trigger

The listener changes a setting under Interface.

## Outcome

The choice applies at once and is kept for later launches.

## Edge cases

- A settings write failure is reported.
