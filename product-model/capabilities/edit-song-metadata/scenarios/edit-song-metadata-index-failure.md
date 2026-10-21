---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener saves metadata.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product writes the song fields but reports that library reconciliation failed.
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
  actor: listener
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

# Report saved tags awaiting library reconciliation

## Trigger

The audio file write succeeds but the following index update fails.

## Outcome

Orca distinguishes saved metadata from the failed library update and retains recovery information for a subsequent startup or edit.
