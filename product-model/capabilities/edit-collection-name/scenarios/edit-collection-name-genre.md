---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener enters a new name in the genre’s editor and saves.
  kind: actor
  entities:
  - entity: genre
    effect: reads
    facts:
    - Name
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product checks that every song file of the genre can be written before changing any.
  kind: product
  entities:
  - entity: genre
    effect: reads
    facts:
    - Name
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
- text: The Product renames the genre and writes the new genre tag into each of its songs.
  kind: product
  entities:
  - entity: genre
    effect: changes
    facts:
    - Name
  - entity: song
    effect: changes
    facts:
    - Genre
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

# Rename a genre

## Trigger

The listener renames a genre with Edit details.

## Outcome

Every active song of the genre carries the new genre tag, and the genre keeps its cover; other tags, lyrics and embedded pictures are unchanged.
