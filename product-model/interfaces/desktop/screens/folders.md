---
entities:
- entity: folder
  shows:
  - Path
  - Name
  - Songs
  - Subfolders
  - Cover
- entity: music-folder
  shows:
  - Path
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
  target: crates/orca-services/src/folders.rs
- kind: code
  role: implementation
  target: app/ui/folders.slint
---

# Folders

Configured roots and nested folders, with indexed songs and child folders.
