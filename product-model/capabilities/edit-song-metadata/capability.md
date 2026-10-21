---
availability:
- place: desktop
references:
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs#write_metadata
- kind: code
  role: implementation
  target: app/src/media/metadata.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/library/metadata.rs#update_song_metadata_checked
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
---

# Edit song metadata

Edit a song’s tags, lyrics and embedded cover, import LRC or text lyrics, and save them to the audio file and the library.
