---
availability:
- place: desktop
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/playlists.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/db/playlists.rs
- kind: code
  role: implementation
  target: app/src/controllers/playlists.rs
domain: playlists
---

# Export playlist

Write a saved playlist’s active songs to an extended M3U file in membership order.
