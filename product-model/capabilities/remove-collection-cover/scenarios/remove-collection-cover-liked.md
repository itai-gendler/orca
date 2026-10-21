---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Remove cover for Liked Songs.
  kind: actor
  entities:
  - entity: song
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product removes the custom Liked Songs cover from the profile.
  kind: product
  entities:
  - entity: preferences
    effect: changes
    facts:
    - Liked Songs cover
  - entity: song
    effect: reads
    facts: []
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

# Remove the Liked Songs cover

## Trigger

The listener removes the custom Liked Songs cover from its menu.

## Outcome

Liked Songs shows artwork from its songs again; likes and audio files are unchanged.
