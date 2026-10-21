---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener saves a new name for an album.
  kind: actor
  entities:
  - entity: album
    effect: reads
    facts:
    - Title
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product finds a song file it cannot write and stops before changing any file.
  kind: product
  entities:
  - entity: album
    effect: reads
    facts:
    - Title
    - Album artist
    - Songs
    - Cover
  - entity: song
    effect: reads
    facts:
    - Path
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/collection_names.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/library/metadata.rs
- kind: code
  role: implementation
  target: app/src/controllers/collections.rs
---

# Stop before writing when a song file is unavailable

## Trigger

One of the album’s song files is missing, unreadable, read-only or changed while being checked.

## Outcome

No file is changed, and Orca says which problem stopped the rename.
