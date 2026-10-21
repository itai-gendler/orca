---
appliesTo:
- type: entity
  id: album
- type: entity
  id: song
  facts:
  - Album
  - Album artist
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/catalog.rs
---

# Album identity includes album artist

Releases with the same title and different album artists remain distinct collections.
