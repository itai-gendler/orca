---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener switches automatic artwork fetching on or off.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::library
- text: The Product saves preferences for the profile.
  kind: product
  entities:
  - entity: preferences
    effect: changes
    facts:
    - Automatic artwork
  actor: listener
  contexts:
    desktop:
      place: desktop::library
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

# Change Library settings

## Trigger

The listener changes the automatic artwork setting under Library.

## Outcome

The choice applies at once and is kept for later launches.

## Edge cases

- A settings write failure is reported.
