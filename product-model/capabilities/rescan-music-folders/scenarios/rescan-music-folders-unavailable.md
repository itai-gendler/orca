---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Rescan for a disconnected music folder.
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
- text: The Product marks the music folder unavailable and reports the scan failure.
  kind: product
  entities:
  - entity: music-folder
    effect: changes
    facts:
    - Available
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

# Keep an unavailable music root

## Trigger

The listener requests a rescan of a disconnected source.

## Outcome

The root and its indexed songs remain; an unavailable-folder error asks the listener to reconnect or remove the root.
