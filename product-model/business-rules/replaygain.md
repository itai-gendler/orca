---
appliesTo:
- type: entity
  id: preferences
  facts:
  - ReplayGain
- type: entity
  id: playback-session
references:
- kind: code
  role: implementation
  target: app/ui/settings.slint
- kind: code
  role: implementation
  target: crates/orca-core/src/audio_engine/worker.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/audio_engine/sources.rs#track_replaygain
---

# ReplayGain leaves untagged tracks unchanged

When enabled, playback uses available track loudness tags; music without them plays without gain adjustment.
