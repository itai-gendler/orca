---
entities:
- entity: playlist
  shows:
  - Name
  - Songs
  - Cover
- entity: song
  shows:
  - Title
  - Artist
  - Album
  - Album artist
  - Duration
  - Quality
  - Cover
  - Explicit
  - Liked
- entity: preferences
  shows:
  - Liked Songs cover
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/catalog.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/media/playlists.rs
- kind: code
  role: implementation
  target: app/ui/library.slint
---

# Playlists

Saved playlists and Liked Songs, with the selected collection and its active members.
