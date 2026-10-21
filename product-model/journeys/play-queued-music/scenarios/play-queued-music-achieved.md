---
kind: primary
result: achieved
routes:
  desktop: Orca desktop
steps:
- text: The listener requests an explicit addition of the selected album.
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
  capability: add-to-queue
- text: The Product adds tracks to the queue and opens its preview.
  kind: product
  entities:
  - entity: queue
    effect: changes
    facts:
    - Songs
    - Priority songs
  actor: listener
  contexts:
    desktop:
      place: desktop::queue
  capability: add-to-queue
- text: The listener selects a queued song.
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
  capability: play-songs
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
  capability: play-songs
references:
- kind: code
  role: implementation
  target: app/ui/library.slint
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
---

# Play from the opened queue

## Trigger

The listener adds a collection to the queue while music is playing.

## Outcome

Orca opens Queue; the listener chooses one of its tracks and playback uses that existing context.
