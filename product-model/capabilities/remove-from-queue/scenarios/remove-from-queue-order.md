---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener removes an upcoming track from Queue with the Delete key or a right-click.
  kind: actor
  entities:
  - entity: queue
    effect: reads
    facts:
    - Songs
  actor: listener
  contexts:
    desktop:
      place: desktop::queue
- text: The Product drops the track from the upcoming order and from explicit additions.
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
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
---

# Remove from queue

## Trigger

The listener removes an upcoming track in Queue.

## Outcome

The track no longer plays in this listening order; music files and playlists are unchanged.

## Edge cases

- The current track stays in place and cannot be removed this way.
