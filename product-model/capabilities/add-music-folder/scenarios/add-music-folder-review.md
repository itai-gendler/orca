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
- text: The Product presents the selected root and child directories for review.
  kind: product
  entities:
  - entity: folder
    effect: reads
    facts:
    - Path
    - Name
    - Subfolders
  actor: listener
  contexts:
    desktop:
      place: desktop::folder-review
- text: The listener finishes the review.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::folder-review
- text: The Product saves the music folder and its folder rules, then schedules scanning.
  kind: product
  entities:
  - entity: music-folder
    effect: creates
    facts:
    - Path
    - Available
    - Scan progress
  - entity: folder-rule
    effect: creates
    facts:
    - Path
    - Mode
    with: music-folder
  actor: listener
  contexts:
    desktop:
      place: desktop::folder-review
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

# Add a reviewed music root

## Trigger

The listener chooses Add folder in Library settings.

## Outcome

The root and reviewed inclusion choices are saved before its initial scan starts.

## Edge cases

- Cancelling review leaves the prospective root unregistered.
- An already configured root opens management rather than creating a duplicate.
