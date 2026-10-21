---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Delete playlist on a saved playlist.
  kind: actor
  entities:
  - entity: playlist
    effect: reads
    facts:
    - Name
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product deletes the playlist and its memberships.
  kind: product
  entities:
  - entity: playlist
    effect: removes
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
  target: app/src/controllers/playlists.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
---

# Delete a playlist

## Trigger

The listener chooses Delete playlist from a playlist’s menu.

## Outcome

The playlist is gone and Orca returns to Playlists; its songs, their files and their likes remain.
