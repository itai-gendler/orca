---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener adds a song to what plays next from its menu.
  kind: actor
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Album artist
    - Duration
    - Quality
    - Cover
    - Explicit
  actor: listener
  contexts:
    desktop:
      place: desktop::songs
- text: The Product puts the track at the end of the explicit additions in the queue.
  kind: product
  entities:
  - entity: queue
    effect: changes
    facts:
    - Priority songs
    - Songs
  actor: listener
  contexts:
    desktop:
      place: desktop::queue
references:
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
---

# Queue an individual track

## Trigger

The listener chooses Add to queue for a track.

## Outcome

The track becomes a priority addition. Adding it again moves that explicit addition to the end rather than duplicating it.
