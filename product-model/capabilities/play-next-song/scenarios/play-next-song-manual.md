---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Next, presses Alt+N, swipes the player or uses the tray or Windows media controls.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product consumes the queue addition and advances the playback session.
  kind: product
  entities:
  - entity: queue
    effect: changes
    facts:
    - Songs
    - Priority songs
    - History
  - entity: playback-session
    effect: changes
    from: Playing
    to: Playing
    facts:
    - Current song
    - Position
  actor: listener
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/platform/media_session.rs
---

# Advance to an explicitly queued track

## Trigger

The listener requests Next while explicit additions are waiting.

## Outcome

The oldest eligible priority track plays, including under Repeat One; the remaining collection continues after additions are consumed.
