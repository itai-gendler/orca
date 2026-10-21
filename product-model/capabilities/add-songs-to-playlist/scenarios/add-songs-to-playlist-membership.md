---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener picks a saved playlist from a song’s Add to playlist choices.
  kind: actor
  entities:
  - entity: playlist
    effect: reads
    facts:
    - Name
    - Songs
    - Cover
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Album artist
    - Duration
    - Quality
    - Cover
    - Explicit
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product appends it to the playlist once.
  kind: product
  entities:
  - entity: playlist
    effect: changes
    facts:
    - Songs
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
  target: crates/orca-core/src/db/playlists.rs
- kind: code
  role: implementation
  target: app/src/controllers/playlists.rs
---

# Add a song to a playlist

## Trigger

The listener chooses a playlist from a song’s menu or from the player’s Add to playlist picker.

## Outcome

The song is added at the end of the playlist.

## Edge cases

- A song already in the playlist keeps its single membership.
