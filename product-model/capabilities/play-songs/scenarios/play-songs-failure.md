---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener selects a song.
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
- text: The Product reports the audio failure and stops the playback session.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    from: Playing
    to: Stopped
    facts: []
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

# Report an unreadable audio file

## Trigger

The selected audio cannot be opened or decoded.

## Outcome

Playback stops advancing and an error is shown; another selection can be attempted.
