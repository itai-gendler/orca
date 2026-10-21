---
entities:
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

# Songs

The active local music catalog, with search, sorting and alphabetical browsing.
