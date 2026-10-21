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
- text: The Product reports that it could not connect.
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

# Report a lyric provider connection failure

## Trigger

The explicit editor lookup cannot reach a provider.

## Outcome

A connection error is shown; saved lyrics and the audio file remain unchanged.
