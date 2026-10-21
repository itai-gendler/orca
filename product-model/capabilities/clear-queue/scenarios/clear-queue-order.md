---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Clear in Queue.
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
- text: The Product drops every upcoming track and explicit addition, keeping the current track.
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

# Clear queue

## Trigger

The listener chooses Clear in Queue.

## Outcome

Only the current track is left in the listening order; music files and playlists are unchanged.
