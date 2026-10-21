---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Remove cover in the metadata editor, then Save metadata.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product removes the embedded pictures from the audio file and refreshes the song cover in the library.
  kind: product
  entities:
  - entity: song
    effect: changes
    facts:
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
  target: crates/orca-core/src/library/metadata.rs
---

# Remove a song's embedded cover

## Trigger

The listener removes the cover in the metadata editor and saves.

## Outcome

The audio file no longer holds embedded pictures, and the library shows the song without its cover.

## Edge cases

- Closing the editor without saving leaves the embedded cover unchanged.
