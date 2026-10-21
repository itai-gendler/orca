---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects a root directory.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::library
- text: The Product saves the music folder and schedules scanning.
  kind: product
  entities:
  - entity: music-folder
    effect: creates
    facts:
    - Path
    - Available
    - Scan progress
  actor: listener
  contexts:
    desktop:
      place: desktop::library
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
- kind: code
  role: implementation
  target: app/src/controllers/folders.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
---

# Add a root without subfolders

## Trigger

The chosen root has no child directories.

## Outcome

Orca saves the root and starts scanning without a separate review.
