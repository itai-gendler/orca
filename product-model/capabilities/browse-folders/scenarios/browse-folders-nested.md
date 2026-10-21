---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects a folder.
  kind: actor
  entities:
  - entity: folder
    effect: reads
    facts: &id001
    - Path
    - Name
    - Songs
    - Subfolders
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::folders
- text: The Product presents music and children under the selected path.
  kind: product
  entities:
  - entity: folder
    effect: reads
    facts: *id001
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
      place: desktop::folders
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
- kind: code
  role: implementation
  target: crates/orca-services/src/folders.rs
---

# Browse a nested music directory

## Trigger

The listener selects a configured root or child directory.

## Outcome

The selected directory shows its indexed descendant songs and child folders; an empty configured root remains reachable.
