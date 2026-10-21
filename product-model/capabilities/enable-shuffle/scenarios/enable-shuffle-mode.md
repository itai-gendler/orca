---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener turns shuffle on in the player.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product turns playback session shuffle on.
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

# Turn shuffle on

## Trigger

The listener wants the listening context in random order.

## Outcome

Next picks tracks of the listening context that have not played yet; a track already prepared for gapless playback is chosen again.
