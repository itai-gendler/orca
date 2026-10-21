---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens Artists or types a search.
  kind: actor
  entities:
  - entity: artist
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::artists
- text: The Product lists the matching artists.
  kind: product
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

# Browse artists in the local catalog

## Trigger

The listener opens Artists or changes its search.

## Outcome

Artists whose name matches the search are listed with their song count and cover.

## Edge cases

- An empty match presents no matching items.
- Results from an earlier search do not replace the current search.
