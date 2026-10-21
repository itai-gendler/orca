---
kind: validation
routes:
  desktop: Orca desktop
steps:
- text: The listener submits an empty name.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::playlists
- text: The Product requests a nonempty name.
  kind: product
  entities: []
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

# Reject a blank playlist name

## Trigger

The listener submits a blank or whitespace-only name.

## Outcome

The name is rejected and the editor remains open.
