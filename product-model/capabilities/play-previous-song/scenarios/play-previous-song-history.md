---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Previous, presses Alt+P, swipes the player or uses the tray or Windows media controls.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product retraces queue history and changes the playback session.
  kind: product
  entities:
  - entity: queue
    effect: changes
    facts:
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
  target: app/src/platform/media_session.rs
---

# Retrace shuffle listening history

## Trigger

The listener requests Previous after hearing multiple shuffled tracks.

## Outcome

The prior nonremoved history track plays; a later Next can retrace forward unless a priority addition takes precedence.

## Edge cases

- With shuffle off, Previous follows collection order; Repeat All can wrap to the last track.
- With no eligible predecessor the current selection remains.
