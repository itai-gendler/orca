---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens lyrics.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product presents playback session lyrics at the current position.
  kind: product
  entities:
  - entity: playback-session
    effect: reads
    facts:
    - Position
    - Displayed lyrics
  actor: listener
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/lyrics.rs
- kind: code
  role: implementation
  target: app/src/media/lyrics.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
- kind: code
  role: implementation
  target: app/src/worker/background.rs
---

# Read lyrics with playback timing

## Trigger

The listener opens lyrics for the current track.

## Outcome

Plain text remains readable; timed lines and words follow playback position.
