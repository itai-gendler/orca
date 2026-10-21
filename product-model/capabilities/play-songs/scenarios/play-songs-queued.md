---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses a queued song.
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
  - entity: queue
    effect: reads
    facts:
    - Songs
  actor: listener
  contexts:
    desktop:
      place: desktop::queue
- text: The Product changes the playback session and records queue history.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    from: Playing
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

# Play a queued track without replacing the context

## Trigger

The listener chooses a track in Queue.

## Outcome

The chosen track plays within the existing queue and history.
