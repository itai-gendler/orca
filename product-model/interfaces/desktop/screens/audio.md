---
entities:
- entity: preferences
  shows:
  - Gapless playback
  - ReplayGain
  - Quality info
  collects:
  - Gapless playback
  - ReplayGain
  - Quality info
references:
- kind: code
  role: implementation
  target: app/ui/settings.slint
- kind: code
  role: implementation
  target: app/src/controllers/preferences.rs
---

# Audio

The profile’s audio settings.
