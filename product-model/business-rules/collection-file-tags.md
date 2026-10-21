---
appliesTo:
- type: entity
  id: artist
  facts:
  - Name
- type: entity
  id: album
  facts:
  - Title
- type: entity
  id: genre
  facts:
  - Name
- type: entity
  id: song
  facts:
  - Artist
  - Album artist
  - Album
  - Genre
references:
- kind: code
  role: implementation
  target: crates/orca-core/src/library/metadata.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/collection_names.rs
---

# Collection renames change matching song tags

Album, artist and genre names are written to the active member files. Matching album-artist credits follow an artist rename; unrelated credits, lyrics and embedded artwork remain.
