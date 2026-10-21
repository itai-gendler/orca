---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener enters a new name in the album’s editor and saves.
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
- text: The Product checks that every song file of the album can be written before changing any.
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
- text: The Product renames the album and writes the new album tag into each of its songs.
  kind: product
  entities:
  - entity: album
    effect: changes
    facts:
    - Title
  - entity: song
    effect: changes
    facts:
    - Album
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

# Rename an album

## Trigger

The listener renames an album with Edit details.

## Outcome

Every active song of the album carries the new album tag, and the album keeps its cover; other tags, lyrics and embedded pictures are unchanged.
