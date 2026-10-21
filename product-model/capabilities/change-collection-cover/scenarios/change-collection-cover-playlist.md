---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener picks an image as the playlist’s cover.
  kind: actor
  entities:
  - entity: playlist
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::collection-editor
- text: The Product saves the image as the playlist’s cover in the profile.
  kind: product
  entities:
  - entity: playlist
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

# Change a playlist’s cover

## Trigger

The listener chooses Change cover from the playlist’s menu, or replaces the cover in its editor and saves.

## Outcome

The playlist shows the new cover; the embedded covers of its songs are unchanged.

## Edge cases

- Invalid or unreadable image input reports an error before the edit is saved.
