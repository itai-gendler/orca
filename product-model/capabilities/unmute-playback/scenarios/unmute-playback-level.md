---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener presses M or clicks the volume button while muted.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product restores the playback session volume to the remembered level.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    facts:
    - Volume
  actor: listener
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/controllers/input.rs
---

# Unmute playback

## Trigger

Playback is muted.

## Outcome

Sound returns at the level it had before muting.
