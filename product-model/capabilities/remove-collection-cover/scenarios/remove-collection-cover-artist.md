---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Remove cover for the artist.
  kind: actor
  entities:
  - entity: artist
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product removes the artist’s custom cover from the profile.
  kind: product
  entities:
  - entity: artist
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

# Remove an artist’s cover

## Trigger

The listener chooses Remove cover from the artist’s menu, or removes it in its editor and saves.

## Outcome

The custom cover, including one fetched online, is gone and the artist falls back to artwork from its songs where there is any; embedded song covers are unchanged.
