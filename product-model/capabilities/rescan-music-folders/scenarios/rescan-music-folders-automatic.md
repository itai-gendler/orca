---
kind: background
routes:
  desktop: Orca desktop
steps:
- text: Startup or a filesystem change schedules a music folder scan.
  kind: condition
  entities:
  - entity: music-folder
    effect: reads
    facts:
    - Path
  unattended: true
  contexts:
    desktop:
      place: desktop::library
- text: The Product indexes newly discovered songs and refreshes changed music.
  kind: product
  entities:
  - entity: music-folder
    effect: changes
    facts:
    - Available
    - Scan progress
  - entity: song
    effect: creates
    to: Indexed
    facts:
    - Path
    - Title
    - Artist
    - Album
    - Album artist
    - Year
    - Track number
    - Disc number
    - Genre
    - Duration
    - Quality
    - Cover
    - Lyrics
    - Lyrics removed
    - Liked
    - Explicit
  contexts:
    desktop:
      place: desktop::library
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/scanning.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/service/watcher.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/scanner.rs
---

# Reconcile music after startup or file changes

## Trigger

Startup finds reachable roots, or a watched root changes.

## Outcome

The same inclusion and indexing policies refresh the local catalog.
