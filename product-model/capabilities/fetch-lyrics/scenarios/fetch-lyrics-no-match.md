---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Fetch lyrics in the metadata editor.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product reports no matching result.
  kind: product
  entities: []
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
  target: crates/orca-services/src/operations/media.rs
- kind: code
  role: implementation
  target: app/src/runtime/operations.rs
---

# Report no matching lyrics

## Trigger

The provider finds no acceptable lyric result.

## Outcome

The editor reports that no lyrics were found; nothing is saved automatically.
