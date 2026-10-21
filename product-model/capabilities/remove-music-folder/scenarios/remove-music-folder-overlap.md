---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener removes the overlapping music folder.
  kind: actor
  entities:
  - entity: music-folder
    effect: reads
    facts:
    - Path
  actor: listener
  contexts:
    desktop:
      place: desktop::library
- text: The Product removes that music folder and retains records covered elsewhere.
  kind: product
  entities:
  - entity: music-folder
    effect: removes
  - entity: song
    effect: reads
    facts:
    - Path
  - entity: folder-rule
    effect: reads
    facts:
    - Path
    - Mode
  actor: listener
  contexts:
    desktop:
      place: desktop::library
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
---

# Retain songs covered by another root

## Trigger

A removed root overlaps another configured root.

## Outcome

Songs and rules still covered by a remaining root survive.
