---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener requests artwork for the selected album.
  kind: actor
  entities:
  - entity: album
    effect: reads
    facts:
    - Title
    - Album artist
    - Songs
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product returns artwork to the current draft.
  kind: product
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/artwork.rs
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
- kind: code
  role: implementation
  target: app/src/controllers/collections.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
- kind: code
  role: implementation
  target: app/src/runtime/operations.rs
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
---

# Stage a collection artwork result

## Trigger

The listener requests artwork while editing an album or artist.

## Outcome

The matched result is staged in that editor; it changes saved artwork only when the listener saves.

## Edge cases

- A result for a closed or superseded editor is ignored.
- Changed song search tags require a new artwork search.
