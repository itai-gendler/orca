---
kind: validation
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
- text: The Product reports no library matches.
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

# Report an import with no library matches

## Trigger

The selected playlist has no paths matching indexed songs.

## Outcome

No playlist is created and Orca reports that no tracks are in the library.
