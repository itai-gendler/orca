---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens Albums or types a search.
  kind: actor
  entities:
  - entity: album
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::albums
- text: The Product lists the matching albums.
  kind: product
  entities:
  - entity: album
    effect: reads
    facts:
    - Title
    - Album artist
    - Songs
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::albums
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

# Browse albums in the local catalog

## Trigger

The listener opens Albums or changes its search.

## Outcome

Albums whose title or album artist matches the search are listed with their album artist, song count and cover.

## Edge cases

- An empty match presents no matching items.
- Results from an earlier search do not replace the current search.
