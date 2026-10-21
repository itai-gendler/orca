---
entities:
- entity: song
  shows:
  - Title
  - Artist
  - Album
  - Album artist
  - Year
  - Track number
  - Disc number
  - Genre
  - Cover
  - Lyrics
  collects:
  - Title
  - Artist
  - Album
  - Album artist
  - Year
  - Track number
  - Disc number
  - Genre
  - Cover
  - Lyrics
references:
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
---

# Metadata editor

The selected audio file and its editable tags, artwork and lyric text.
