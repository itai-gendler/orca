---
appliesTo:
- type: entity
  id: song
  effect: changes
  to: Excluded
- type: entity
  id: playlist
  facts:
  - Songs
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/db/schema.rs
---

# Exclusion retains song identity and memberships

Excluded songs disappear from active browsing and playback while their indexed identity, likes and playlist memberships remain for reinclusion.
