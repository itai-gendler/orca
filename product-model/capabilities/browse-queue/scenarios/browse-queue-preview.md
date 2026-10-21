---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens Queue from the player or with the Q key.
  kind: actor
  entities:
  - entity: queue
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::queue
- text: The Product shows the current track and the upcoming tracks of the queue.
  kind: product
  entities:
  - entity: queue
    effect: reads
    facts:
    - Songs
    - Priority songs
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
      place: desktop::queue
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
---

# Inspect the listening preview

## Trigger

The listener opens Queue.

## Outcome

Queue lists the current track and up to 20 upcoming ones; with shuffle on, it says so rather than predicting the shuffled order.
