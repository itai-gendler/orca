---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener selects a collection.
  kind: actor
  entities:
  - entity: artist
    effect: reads
    facts: &id001
    - Name
    - Songs
    - Albums
    - Cover
  actor: listener
  contexts:
    desktop:
      place: desktop::artists
- text: The Product presents its active songs and collection facts.
  kind: product
  entities:
  - entity: artist
    effect: reads
    facts: *id001
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

# Inspect a selected artist

## Trigger

The listener opens one collection.

## Outcome

The selected collection and its active songs are shown; a removed collection returns the listener to browsing.
