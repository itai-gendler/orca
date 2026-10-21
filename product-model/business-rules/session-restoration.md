---
appliesTo:
- type: entity
  id: playback-session
- type: entity
  id: queue
references:
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/service/persistence.rs#read_session
---

# Restoration starts paused in the saved context

Track, position, volume, queue, shuffle and repeat are restored together. A position checkpoint applies only to its matching saved context and track.
