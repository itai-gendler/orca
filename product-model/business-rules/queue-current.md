---
appliesTo:
- type: entity
  id: queue
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
---

# Queue removal and reordering preserve the current track

Removing, clearing or moving queue items does not remove or move the currently playing track.
