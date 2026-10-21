---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens Genres or types a search.
  kind: actor
  entities:
  - entity: genre
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::genres
- text: The Product lists the matching genres.
  kind: product
  entities:
  - entity: genre
    effect: reads
    facts:
    - Name
    - Songs
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::genres
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

# Browse genres in the local catalog

## Trigger

The listener opens Genres or changes its search.

## Outcome

Genres whose name matches the search are listed with their song count and cover.

## Edge cases

- An empty match presents no matching items.
- Results from an earlier search do not replace the current search.
