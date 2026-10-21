---
kind: validation
routes:
  desktop: Orca desktop
steps:
- text: The listener saves a blank name, or a name another album with the same release credit already has.
  kind: actor
  entities:
  - entity: album
    effect: reads
    facts:
    - Title
    - Album artist
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product asks for a different name and changes no file.
  kind: product
  entities: []
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

# Reject a blank or conflicting collection name

## Trigger

The new name is blank, or another album, artist or genre of that name already exists.

## Outcome

Orca asks for a name, or for a different one, and no song file is changed.
