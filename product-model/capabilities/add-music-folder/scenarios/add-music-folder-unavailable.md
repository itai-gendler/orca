---
kind: validation
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses a location.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::library
- text: The Product reports that the selected directory is unavailable.
  kind: product
  entities: []
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

# Reject an unavailable directory

## Trigger

The chosen root cannot be read as a directory.

## Outcome

An error is shown and no new root is registered.
