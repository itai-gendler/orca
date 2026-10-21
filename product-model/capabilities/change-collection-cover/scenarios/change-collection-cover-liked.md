---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Change cover for Liked Songs and picks an image.
  kind: actor
  entities:
  - entity: song
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product changes the profile artwork for the liked selection.
  kind: product
  entities:
  - entity: preferences
    effect: changes
    facts:
    - Liked Songs cover
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
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

# Change collection cover for Liked Songs

## Trigger

The listener changes the Liked Songs cover from its menu.

## Outcome

The custom artwork is saved in the local profile without changing liked memberships or audio files.
