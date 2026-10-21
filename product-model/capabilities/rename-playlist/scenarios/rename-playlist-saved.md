---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener enters a new name for a saved playlist and saves.
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
- text: The Product renames the playlist.
  kind: product
  entities:
  - entity: playlist
    effect: changes
    facts:
    - Name
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

# Rename a playlist

## Trigger

The listener renames a playlist from its menu or its editor.

## Outcome

The playlist appears under its new name; song tags are unchanged.
