---
kind: validation
routes:
  desktop: Orca desktop
steps:
- text: The listener submits edited fields.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product reports invalid metadata and retains the editor.
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

# Reject incomplete or invalid tags

## Trigger

The listener submits blank required fields or invalid numeric fields.

## Outcome

The editor requests title, artist, album and album artist; year, track and disc must be blank or positive whole numbers. Nothing is written.
