---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener opens Songs, types a search or picks a sort.
  kind: actor
  entities:
  - entity: song
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::songs
- text: The Product lists the matching active songs in the chosen order.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Album artist
    - Duration
    - Quality
    - Cover
    - Explicit
  actor: listener
  contexts:
    desktop:
      place: desktop::songs
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

# Browse songs in the local catalog

## Trigger

The listener opens Songs, changes its search or picks a sort.

## Outcome

Active songs whose title, artist, album or format matches the search are listed by title, artist, album or most recently added.

## Edge cases

- An empty match presents no matching items.
- Results from an earlier search do not replace the current search.
- Choosing a letter jumps to the first song at that letter in the current order.
