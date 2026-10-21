---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Pause in the player, presses Space, or uses the tray or Windows media controls.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product pauses the playback session.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    from: Playing
    to: Paused
    facts: []
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
  target: app/src/controllers/input.rs
- kind: code
  role: implementation
  target: app/src/platform/platform_integration.rs
- kind: code
  role: implementation
  target: app/src/platform/media_session.rs
---

# Pause the playing track

## Trigger

Music is playing.

## Outcome

The track stays loaded at its position.
