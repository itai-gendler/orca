---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens Folders or types a search.
  kind: actor
  entities:
  - entity: folder
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::folders
- text: The Product lists the matching folders.
  kind: product
  entities:
  - entity: folder
    effect: reads
    facts:
    - Path
    - Name
    - Songs
    - Subfolders
    - Cover
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

# Browse folders in the local catalog

## Trigger

The listener opens Folders or changes its search.

## Outcome

The configured music roots are listed, each opening onto its nested folders and songs.

## Edge cases

- An empty match presents no matching items.
- Results from an earlier search do not replace the current search.
