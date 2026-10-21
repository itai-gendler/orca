---
appliesTo:
- type: entity
  id: playlist
references:
- kind: code
  role: implementation
  target: crates/orca-core/src/db/playlists.rs
---

# A playlist contains each song at most once

New members append in order; repeat additions do not duplicate memberships. Removing memberships or deleting playlists preserves audio files.
