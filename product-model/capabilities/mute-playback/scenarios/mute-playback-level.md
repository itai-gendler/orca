---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener presses M or clicks the volume button while sound is audible.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product sets the playback session volume to zero.
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

# Mute playback

## Trigger

The listener wants silence without pausing.

## Outcome

Playback continues silently, and the previous level is remembered for unmuting.
