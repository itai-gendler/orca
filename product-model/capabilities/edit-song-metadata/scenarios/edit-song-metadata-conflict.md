---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener saves the draft.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product reports that the source changed and keeps the draft open.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Path
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

# Reopen a file changed outside the editor

## Trigger

The file has changed since the editor read it.

## Outcome

Save reports a conflict and asks the listener to reopen the editor.
