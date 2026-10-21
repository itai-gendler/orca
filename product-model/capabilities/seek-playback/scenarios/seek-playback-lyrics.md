---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener clicks a timed lyric line.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product changes the playback session position.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    facts:
    - Position
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
  target: app/src/main.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
---

# Seek to a timed lyric

## Trigger

The listener selects a lyric line that carries a timestamp.

## Outcome

The current track jumps to that line, and the lyrics follow from there.

## Edge cases

- Untimed lyric lines do not provide a seek target.
