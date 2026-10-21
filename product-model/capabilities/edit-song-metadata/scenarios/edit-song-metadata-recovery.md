---
kind: background
routes:
  desktop: Orca desktop
steps:
- text: An interrupted song edit requires reconciliation.
  kind: condition
  entities:
  - entity: song
    effect: reads
    facts:
    - Path
  unattended: true
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product refreshes song facts from the actual file.
  kind: product
  entities:
  - entity: song
    effect: changes
    facts:
    - Title
    - Artist
    - Album
    - Album artist
    - Year
    - Track number
    - Disc number
    - Genre
    - Lyrics
    - Lyrics removed
    - Cover
  contexts:
    desktop:
      place: desktop::metadata-editor
references:
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs
- kind: code
  role: implementation
  target: app/src/media/metadata.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/library/metadata.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
---

# Reconcile the file that committed

## Trigger

A later startup or edit finds an interrupted metadata save.

## Outcome

The index is reconciled from the committed file rather than from the obsolete draft.
