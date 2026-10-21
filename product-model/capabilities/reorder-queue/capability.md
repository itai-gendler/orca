---
availability:
- place: desktop
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
domain: queue
---

# Reorder queue

Move an upcoming track earlier in Queue; the current track stays where it is.
