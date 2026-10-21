---
availability:
- place: desktop
references:
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/service/persistence.rs
---

# Restore playback session

Restore saved track, position, queue, volume, shuffle and repeat when Orca starts, with audio paused.
