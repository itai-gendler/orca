---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener selects an M3U exchange file.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product reports the file read failure.
  kind: product
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/playlists.rs
- kind: code
  role: implementation
  target: app/src/controllers/playlists.rs
---

# Report a playlist file read failure

## Trigger

The selected exchange file cannot be read.

## Outcome

Import reports an error without creating a partial playlist.
