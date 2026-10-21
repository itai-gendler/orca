---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens the selected song for editing.
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
- text: The Product rereads the audio file and presents its current tags, cover and lyrics.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Album artist
    - Year
    - Track number
    - Disc number
    - Genre
    - Cover
    - Lyrics
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The listener edits tags or lyrics, or imports an LRC or text file, and chooses Save metadata.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product saves song tags, lyrics and any chosen cover, then refreshes the catalog.
  kind: product
  entities:
  - entity: song
    effect: changes
    facts:
    - Title
    - Artist
    - Album
    - Album artist
    - Year
    - Track number
    - Disc number
    - Genre
    - Lyrics
    - Lyrics removed
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
  target: app/src/media/metadata.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/library/metadata.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
---

# Save edited tags and imported lyrics

## Trigger

The listener chooses Edit metadata on an indexed song.

## Outcome

Validated tags and chosen assets are written to the audio file, the index is refreshed, and the matching editor closes.

## Edge cases

- Cancelling discards unsaved edits.
- Import accepts LRC or text without converting it into a separate library item.
- Editing the currently playing file preserves the listening context and refreshes its displayed metadata.
