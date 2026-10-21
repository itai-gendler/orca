---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener enters a new name in the artist’s editor and saves.
  kind: actor
  entities:
  - entity: artist
    effect: reads
    facts:
    - Name
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product checks that every song file of the artist can be written before changing any.
  kind: product
  entities:
  - entity: artist
    effect: reads
    facts:
    - Name
    - Songs
    - Albums
    - Cover
  - entity: song
    effect: reads
    facts:
    - Path
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product renames the artist and writes the new artist tag into each of its songs.
  kind: product
  entities:
  - entity: artist
    effect: changes
    facts:
    - Name
  - entity: song
    effect: changes
    facts:
    - Artist
    - Album artist
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

# Rename an artist

## Trigger

The listener renames an artist with Edit details.

## Outcome

Every active song credited to the artist carries the new name, and the artist keeps its cover; other tags, lyrics and embedded pictures are unchanged.

## Edge cases

- An artist rename also changes album artist only where it matched the old artist credit.
