---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener drags the seekbar or clicks a point on it.
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

# Seek to a selected position

## Trigger

The listener chooses a point in the current track.

## Outcome

The current track continues from the chosen position, and timed lyrics follow it.

## Edge cases

- A seek failure is reported.
