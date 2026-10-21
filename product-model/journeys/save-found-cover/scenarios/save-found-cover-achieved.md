---
kind: primary
result: achieved
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
  capability: find-cover-online
- text: The Product opens the metadata editor with matching artwork staged.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Album artist
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
  capability: find-cover-online
- text: The listener saves the reviewed metadata.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
  capability: edit-song-metadata
- text: The Product saves the song cover and refreshes its catalog facts.
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
  capability: edit-song-metadata
references:
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
---

# Save the found artwork

## Trigger

The listener chooses Find cover online on a local track.

## Outcome

The listener is carried into the metadata editor and saves the matched artwork into that track.
