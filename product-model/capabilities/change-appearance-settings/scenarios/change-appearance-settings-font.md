---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects a font file.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::appearance
- text: The Product changes the appearance preferences.
  kind: product
  entities:
  - entity: preferences
    effect: changes
    facts:
    - Appearance
  actor: listener
  contexts:
    desktop:
      place: desktop::appearance
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

# Choose a custom font file

## Trigger

The listener selects a supported font file.

## Outcome

The custom font is registered, selected and saved; removing it returns to the preset if it was active.

## Edge cases

- An unavailable saved custom font falls back to the preset and reports an error.
- A file without a supported font is rejected.
