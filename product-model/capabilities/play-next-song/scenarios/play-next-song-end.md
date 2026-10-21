---
kind: background
routes:
  desktop: Orca desktop
steps:
- text: The playback session ends with no eligible next track.
  kind: condition
  entities:
  - entity: playback-session
    effect: reads
    facts:
    - Repeat
  - entity: queue
    effect: reads
    facts:
    - Songs
    - Priority songs
    - History
  unattended: true
  contexts:
    desktop:
      place: desktop::player
- text: The Product leaves the playback session stopped.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    from: Playing
    to: Stopped
    facts: []
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
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/platform/media_session.rs
---

# Finish a non-repeating context

## Trigger

The last track ends, Repeat is Off, and no explicit additions remain.

## Outcome

Playback ends without starting a different collection.
