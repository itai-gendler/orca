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
- text: The Product reports the file write failure.
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

# Report a failed metadata write

## Trigger

The source is unavailable, read-only or cannot preserve the requested tags.

## Outcome

The save failure is reported and the editor stays open.
