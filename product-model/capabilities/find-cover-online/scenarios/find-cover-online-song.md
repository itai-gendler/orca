---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener requests an online cover for a song.
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
- text: The Product opens the metadata editor and searches using song identity.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Album artist
    - Duration
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product presents a matching cover in the draft for Save.
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

# Find a song cover and open its editor

## Trigger

The listener chooses Find cover online on a song.

## Outcome

The metadata editor is opened with the found artwork staged for saving into the audio file.
