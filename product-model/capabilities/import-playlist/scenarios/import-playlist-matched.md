---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects an M3U or M3U8 file.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product matches paths to songs and creates the playlist.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Path
  - entity: playlist
    effect: creates
    facts:
    - Name
    - Songs
    - Cover
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

# Import existing library tracks

## Trigger

The listener chooses a readable playlist file containing indexed paths.

## Outcome

A playlist named after the file is created with unique matched songs in file order; unmatched paths are skipped.

## Edge cases

- Relative paths resolve from the playlist file’s directory.
- Comments, empty lines and duplicate song entries do not become additional members.
- Import does not scan or copy new audio files.
