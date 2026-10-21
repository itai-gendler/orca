---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Rescan for a music folder.
  kind: actor
  entities:
  - entity: music-folder
    effect: reads
    facts:
    - Path
  actor: listener
  contexts:
    desktop:
      place: desktop::library
- text: The Product scans included files and publishes newly indexed songs.
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
  actor: listener
  contexts:
    desktop:
      place: desktop::library
- text: The Product refreshes changed song facts and completes scan progress.
  kind: product
  entities:
  - entity: song
    effect: changes
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
    - Explicit
  - entity: music-folder
    effect: changes
    facts:
    - Scan progress
  actor: listener
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

# Index supported music files

## Trigger

The listener requests Rescan for a root or refreshes the library.

## Outcome

Included supported audio is indexed; existing tracks retain their identity as metadata is refreshed.

## Edge cases

- Unchanged files reuse their indexed metadata.
- Unreadable audio files are skipped with an error.
- MP3, FLAC, M4A, WAV, OGG, Opus, AIFF and AIF files are considered.
