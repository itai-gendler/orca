---
appliesTo:
- type: entity
  id: artist
  facts:
  - Cover
- type: entity
  id: album
  facts:
  - Cover
- type: entity
  id: genre
  facts:
  - Cover
- type: entity
  id: playlist
  facts:
  - Cover
- type: entity
  id: folder
  facts:
  - Cover
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/artwork.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs
---

# Collection covers belong to the local profile

Custom or fetched collection artwork is stored in Orca and does not replace embedded song pictures. Song metadata saves can embed or remove those pictures.
