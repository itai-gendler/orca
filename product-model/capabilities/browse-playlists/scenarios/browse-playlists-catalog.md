---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens Playlists or types a search.
  kind: actor
  entities:
  - entity: playlist
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product lists the matching playlists.
  kind: product
  entities:
  - entity: playlist
    effect: reads
    facts:
    - Name
    - Songs
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
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
---

# Browse playlists in the local catalog

## Trigger

The listener opens Playlists or changes its search.

## Outcome

Liked Songs, while it is shown, and the saved playlists whose name matches the search are listed with their song count and cover.

## Edge cases

- An empty match presents no matching items.
- Results from an earlier search do not replace the current search.
