---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens a folder location.
  kind: actor
  entities:
  - entity: folder
    effect: reads
    facts:
    - Path
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product opens the folder path in Explorer.
  kind: product
  entities:
  - entity: folder
    effect: reads
    facts:
    - Path
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
references:
- kind: code
  role: implementation
  target: app/src/controllers/collections.rs
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
---

# Open the selected directory in Explorer

## Trigger

The listener requests Open in Explorer for a folder.

## Outcome

Windows Explorer opens the directory so the listener can work with it outside Orca.

## Edge cases

- A launch failure is reported.
- Renaming the actual directory is performed in the filesystem.
