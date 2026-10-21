---
kind: background
routes:
  desktop: Orca desktop
steps:
- text: A completed scan identifies an absent included song.
  kind: condition
  entities:
  - entity: song
    effect: reads
    facts:
    - Path
  - entity: music-folder
    effect: reads
    facts:
    - Path
  unattended: true
  contexts:
    desktop:
      place: desktop::library
- text: The Product removes the missing song and its playlist memberships.
  kind: product
  entities:
  - entity: song
    effect: removes
    from: Indexed
  - entity: playlist
    effect: changes
    facts:
    - Songs
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

# Remove records for files no longer present

## Trigger

A complete scan of a reachable root no longer finds an included indexed file.

## Outcome

The missing song and its playlist and like associations are removed; excluded songs are retained.
