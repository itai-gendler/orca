---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects liked songs to remove.
  kind: actor
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Liked
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product clears the song liked flags.
  kind: product
  entities:
  - entity: song
    effect: changes
    facts:
    - Liked
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/playlists.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/db/likes.rs
- kind: code
  role: implementation
  target: app/src/controllers/playlists.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
---

# Remove several tracks from Liked Songs

## Trigger

The listener selects songs in Liked Songs and chooses Remove from Liked Songs.

## Outcome

The selected likes are removed while music files remain.
