---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener switches gapless playback, ReplayGain or track quality info on or off.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::audio
- text: The Product saves preferences for the profile.
  kind: product
  entities:
  - entity: preferences
    effect: changes
    facts:
    - Gapless playback
    - ReplayGain
    - Quality info
  actor: listener
  contexts:
    desktop:
      place: desktop::audio
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

# Change Audio settings

## Trigger

The listener changes a setting under Audio.

## Outcome

The choice applies at once and is kept for later launches.

## Edge cases

- A settings write failure is reported.
