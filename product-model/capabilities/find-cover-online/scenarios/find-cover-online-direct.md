---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener requests artwork for an artist.
  kind: actor
  entities:
  - entity: artist
    effect: reads
    facts:
    - Name
    - Songs
    - Albums
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::artists
- text: The Product saves the matched artist cover.
  kind: product
  entities:
  - entity: artist
    effect: changes
    facts:
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::artists
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/artwork.rs
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
- kind: code
  role: implementation
  target: app/src/controllers/collections.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
- kind: code
  role: implementation
  target: app/src/runtime/operations.rs
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
---

# Apply a direct collection artwork search

## Trigger

The listener requests a cover from an album or artist context without an open editor.

## Outcome

The matching artwork is saved in the profile and collection artwork refreshes.
