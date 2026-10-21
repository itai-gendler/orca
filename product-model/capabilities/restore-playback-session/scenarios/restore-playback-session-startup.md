---
kind: background
routes:
  desktop: Orca desktop
steps:
- text: Orca starts with saved playback session and queue information.
  kind: condition
  entities:
  - entity: playback-session
    effect: reads
    facts:
    - Current song
    - Position
    - Volume
    - Shuffle
    - Repeat
    - Displayed lyrics
  - entity: queue
    effect: reads
    facts: &id001
    - Songs
    - Priority songs
    - History
  unattended: true
  contexts:
    desktop:
      place: desktop::player
- text: The Product restores the playback session and queue.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    from: Stopped
    to: Paused
    facts:
    - Current song
    - Position
    - Volume
    - Shuffle
    - Repeat
  - entity: queue
    effect: changes
    facts: *id001
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/service/persistence.rs
---

# Restore a paused listening session

## Trigger

Orca starts with a saved listening session.

## Outcome

The prior listening context is available with the current track paused at its saved position.

## Edge cases

- No saved session starts with default volume and an empty context.
- An unavailable saved file reports a restoration error.
- A recoverable saved-state backup may supply the prior valid session.
