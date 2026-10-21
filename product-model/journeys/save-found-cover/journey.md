---
actors:
- listener
references:
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
---

# Save an online cover into a song

## Goal

Find artwork for a local track and keep it in the audio file.

## Success criterion

The metadata save completes and the track shows the newly embedded artwork.
