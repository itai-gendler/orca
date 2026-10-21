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
- text: The Product writes the new name into the files before the one that fails.
  kind: product
  entities:
  - entity: song
    effect: changes
    facts:
    - Album
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product reports how far the rename got and refreshes the album list.
  kind: product
  entities:
  - entity: album
    effect: reads
    facts:
    - Title
    - Songs
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

# Report a partly saved album rename

## Trigger

One song file fails to save after earlier songs of the album were saved.

## Outcome

The songs already saved keep the new name. Orca says how far the rename got and asks the listener to reopen the album before retrying.
