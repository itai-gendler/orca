---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener adds the whole selected album to what plays next.
  kind: actor
  entities:
  - entity: album
    effect: reads
    facts:
    - Title
    - Album artist
    - Songs
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::albums
- text: The Product adds the collection’s active tracks to the queue as explicit additions.
  kind: product
  entities:
  - entity: queue
    effect: changes
    facts:
    - Priority songs
    - Songs
  - entity: song
    effect: reads
    facts:
    - Path
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

# Queue a whole collection despite a search

## Trigger

The listener queues a collection while a search is active.

## Outcome

All active tracks in the selected collection are added, disregarding the search; current and already-explicit tracks are skipped.
