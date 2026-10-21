---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener requests artwork for a song.
  kind: actor
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Album artist
    - Duration
    - Quality
    - Cover
    - Explicit
  actor: listener
  contexts:
    desktop:
      place: desktop::songs
- text: The Product reports that no usable artwork was obtained.
  kind: product
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
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

# Keep music usable when lookup fails

## Trigger

Artwork providers fail, have no acceptable match, or cannot identify the intended artist.

## Outcome

Orca reports the lookup result without changing the saved cover or interrupting local music.
