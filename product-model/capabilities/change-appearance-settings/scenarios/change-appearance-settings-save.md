---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener picks a theme, a font, a font size or a visuals preset or option.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::appearance
- text: The Product saves preferences for the profile.
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

# Change Appearance settings

## Trigger

The listener changes a setting under Appearance.

## Outcome

The choice applies at once and is kept for later launches.

## Edge cases

- A settings write failure is reported.
