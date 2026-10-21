---
appliesTo:
- type: entity
  id: music-folder
  facts:
  - Available
- type: entity
  id: song
  effect: removes
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/scanning.rs
---

# An unavailable root is not a deleted song

A scan that cannot reach its configured root retains the root and indexed music; missing-file removal follows successful traversal.
