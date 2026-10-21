---
appliesTo:
- type: capability
  id: find-cover-online
- type: capability
  id: fetch-lyrics
- type: capability
  id: edit-song-metadata
references:
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/artwork.rs
- kind: code
  role: implementation
  target: app/src/runtime/operations.rs
---

# Editor lookups do not change saved music until Save

Manual artwork and lyric results inside editors remain staged. Closing the draft leaves saved music unchanged.
