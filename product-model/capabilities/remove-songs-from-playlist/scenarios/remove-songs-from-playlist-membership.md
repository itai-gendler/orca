---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects songs in a saved playlist and chooses Remove from playlist.
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
- text: The Product removes them from the playlist.
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

# Remove songs from a playlist

## Trigger

The listener removes one song from its menu, several selected songs, or the playing song from the player’s playlist picker.

## Outcome

The songs leave the playlist; their audio files, likes and other playlists are unchanged.
