---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Play in the player, presses Space, or uses the tray or Windows media controls.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product resumes the playback session.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    from: Paused
    to: Playing
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

# Resume the paused track

## Trigger

A track is paused, including one restored at startup.

## Outcome

The same track continues from its position.
