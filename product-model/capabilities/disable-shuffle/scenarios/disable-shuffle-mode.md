---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener turns shuffle off in the player.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product turns playback session shuffle off.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    facts:
    - Shuffle
  actor: listener
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
---

# Turn shuffle off

## Trigger

The listener wants the listening context in order again.

## Outcome

Next and Previous follow the listening context in order from the current track; a track already prepared for gapless playback is chosen again.
