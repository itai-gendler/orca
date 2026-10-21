---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects the playlist and destination.
  kind: actor
  entities:
  - entity: playlist
    effect: reads
    facts:
    - Name
    - Songs
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product writes the playlist representation using active song facts.
  kind: product
  entities:
  - entity: playlist
    effect: reads
    facts:
    - Songs
  - entity: song
    effect: reads
    facts:
    - Path
    - Title
    - Artist
    - Duration
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
  target: crates/orca-core/src/db/playlists.rs
- kind: code
  role: implementation
  target: app/src/controllers/playlists.rs
---

# Export a saved selection

## Trigger

The listener chooses an export destination for a saved playlist.

## Outcome

The file contains track durations, artist/title labels and local audio paths; excluded members are omitted.

## Edge cases

- A failed destination write reports an error.
- Cancelling the destination chooser produces no export.
