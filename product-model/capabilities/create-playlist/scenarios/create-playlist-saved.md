---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses to create a playlist, enters a name and saves.
  kind: actor
  entities:
  - entity: playlist
    effect: reads
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product creates an empty playlist with that name.
  kind: product
  entities:
  - entity: playlist
    effect: creates
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
  target: crates/orca-services/src/media/playlists.rs
- kind: code
  role: implementation
  target: app/src/controllers/playlists.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
---

# Create a playlist

## Trigger

The listener creates a playlist in Playlists.

## Outcome

The new, empty playlist appears in Playlists.
