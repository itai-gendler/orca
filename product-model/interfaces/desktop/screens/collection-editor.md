---
entities:
- entity: artist
  shows:
  - Name
  - Songs
  - Albums
  - Cover
  collects:
  - Name
  - Cover
- entity: album
  shows:
  - Title
  - Album artist
  - Songs
  - Cover
  collects:
  - Title
  - Cover
- entity: genre
  shows:
  - Name
  - Songs
  - Cover
  collects:
  - Name
  - Cover
- entity: playlist
  shows:
  - Name
  - Songs
  - Cover
  collects:
  - Name
  - Cover
- entity: folder
  shows:
  - Path
  - Name
  - Songs
  - Subfolders
  - Cover
  collects:
  - Cover
references:
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
- kind: code
  role: implementation
  target: app/src/controllers/collections.rs
---

# Collection editor

The selected artist, album, genre, playlist or folder and its editable name or custom cover.
