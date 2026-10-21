---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Refresh lyrics for the playing track.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product keeps playback session lyrics and reports the unsuccessful refresh.
  kind: product
  entities:
  - entity: playback-session
    effect: reads
    facts:
    - Displayed lyrics
  actor: listener
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/lyrics.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
---

# Retain displayed lyrics after refresh failure

## Trigger

The refresh fails or returns no usable text while text is already displayed.

## Outcome

Existing displayed text remains and the error is reported.
