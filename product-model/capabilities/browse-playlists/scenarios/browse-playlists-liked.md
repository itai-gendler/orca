---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens the built-in liked selection.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product presents the active liked songs.
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
    - Quality
    - Cover
    - Explicit
    - Liked
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/catalog.rs
- kind: code
  role: implementation
  target: app/src/worker/catalog.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
---

# Read Liked Songs

## Trigger

The listener opens Liked Songs while it is shown.

## Outcome

Active liked tracks appear with newest likes first. Hiding the selection in Interface settings does not discard likes.
