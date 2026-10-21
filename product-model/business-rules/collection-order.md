---
appliesTo:
- type: entity
  id: album
- type: entity
  id: playlist
- type: entity
  id: folder
- type: capability
  id: browse-songs
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/catalog.rs
---

# Collection order follows the selected music context

Albums use track number and title order, playlists use membership order, Liked Songs uses newest-like order, and folders use file-path order. The Songs catalog offers its selected sort.
