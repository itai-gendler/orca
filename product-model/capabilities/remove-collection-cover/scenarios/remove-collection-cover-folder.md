---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Remove cover for the folder.
  kind: actor
  entities:
  - entity: folder
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product removes the folder’s custom cover from the profile.
  kind: product
  entities:
  - entity: folder
    effect: changes
    facts:
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
- kind: code
  role: implementation
  target: app/src/controllers/collections.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/catalog.rs
---

# Remove a folder’s cover

## Trigger

The listener chooses Remove cover from the folder’s menu, or removes it in its editor and saves.

## Outcome

The custom cover, including one fetched online, is gone and the folder falls back to artwork from its songs where there is any; embedded song covers are unchanged.
