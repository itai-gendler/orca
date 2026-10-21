---
kind: background
routes:
  desktop: Orca desktop
steps:
- text: The playback session reaches the track end.
  kind: condition
  entities:
  - entity: playback-session
    effect: reads
    facts:
    - Current song
    - Repeat
    - Shuffle
  unattended: true
  contexts:
    desktop:
      place: desktop::player
- text: The Product selects the next track from the queue and advances the playback session.
  kind: product
  entities:
  - entity: queue
    effect: changes
    facts:
    - Songs
    - Priority songs
    - History
  - entity: playback-session
    effect: changes
    from: Playing
    to: Playing
    facts:
    - Current song
    - Position
  - entity: preferences
    effect: reads
    facts:
    - Gapless playback
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

# Advance after a track finishes

## Trigger

The playing track reaches its end.

## Outcome

An eligible successor plays within the listening context, or the current track repeats under Repeat One.

## Decision points

### Gapless transition

How is the next eligible track loaded?

- Gapless playback enabled → preload the next track near the end and use it on transition.
- Gapless playback disabled → load the next track after the current one ends.
