---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener removes the configured music folder.
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
- text: The Product removes the music folder, its exclusive folder rules and excluded song records.
  kind: product
  entities:
  - entity: music-folder
    effect: removes
  - entity: folder-rule
    effect: removes
    with: music-folder
  - entity: song
    effect: removes
    from: Excluded
  actor: listener
  contexts:
    desktop:
      place: desktop::library
- text: The Product removes the corresponding playlist memberships.
  kind: product
  entities:
  - entity: playlist
    effect: changes
    facts:
    - Songs
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

# Remove retained excluded records with their root

## Trigger

A removed root is the only configured source covering previously excluded music.

## Outcome

The retained excluded records and their saved associations are removed from the profile; files remain on disk.
