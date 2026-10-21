---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener drags the volume slider, scrolls over it or uses the arrow keys on it.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product changes the playback session volume.
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

# Change the volume

## Trigger

The listener adjusts the volume control.

## Outcome

Playback follows the new level, and the level is kept in the saved listening session.
