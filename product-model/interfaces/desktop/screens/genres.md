---
entities:
- entity: genre
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
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/catalog.rs
- kind: code
  role: implementation
  target: app/ui/library.slint
---

# Genres

The genres catalog and the selected collection, its songs and related music.
