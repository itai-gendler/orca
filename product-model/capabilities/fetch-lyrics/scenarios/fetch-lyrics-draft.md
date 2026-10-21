---
kind: primary
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
- text: The Product searches from current song identity and places the result in the draft.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Duration
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

# Fetch fresh lyrics into an editor draft

## Trigger

The listener chooses Fetch lyrics in the metadata editor.

## Outcome

Matching text is staged for Save; the original audio and saved lyrics remain unchanged until that save.

## Edge cases

- A changed track identity or reopened editor cannot receive the older result.
