---
availability:
- place: desktop
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/lyrics.rs
- kind: code
  role: implementation
  target: app/src/media/lyrics.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
- kind: code
  role: implementation
  target: app/src/worker/background.rs
---

# Read lyrics

Read plain, line-synchronized or word-timed lyrics for the current track, using local sources before automatic online lookup.
