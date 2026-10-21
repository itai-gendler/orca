---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener removes a music folder.
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
- text: The Product removes the music folder and its exclusive folder rules and songs.
  kind: product
  entities:
  - entity: music-folder
    effect: removes
  - entity: folder-rule
    effect: removes
    with: music-folder
  - entity: song
    effect: removes
    from: Indexed
  actor: listener
  contexts:
    desktop:
      place: desktop::library
- text: The Product reconciles playlist memberships, queue and playback session.
  kind: product
  entities:
  - entity: playlist
    effect: changes
    facts:
    - Songs
  - entity: queue
    effect: changes
    facts:
    - Songs
    - Priority songs
    - History
  - entity: playback-session
    effect: changes
    from: Playing
    to: Stopped
    facts:
    - Current song
    - Position
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

# Remove an exclusive music source

## Trigger

The listener removes a configured root.

## Outcome

The root and exclusively covered index records are removed. Files remain on disk, and playback is stopped if its current track leaves the active catalog.
