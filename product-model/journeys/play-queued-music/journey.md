---
actors:
- listener
references:
- kind: code
  role: implementation
  target: app/ui/library.slint
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
---

# Listen to a queued collection

## Goal

Add a collection to the listening queue and begin one of its queued tracks.

## Success criterion

A queued track plays in the retained listening context.
