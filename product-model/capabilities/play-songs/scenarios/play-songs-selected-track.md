---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects music to play.
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
- text: The Product starts the playback session and establishes its queue.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    from: Stopped
    to: Playing
    facts:
    - Current song
    - Position
  - entity: queue
    effect: changes
    facts:
    - Songs
    - Priority songs
    - History
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
  target: app/src/playback/navigation.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/audio_engine/mod.rs
---

# Play a track in the current collection

## Trigger

The listener chooses a song or Play on a collection.

## Outcome

Audio starts and navigation follows the selected filtered collection in its current order.
