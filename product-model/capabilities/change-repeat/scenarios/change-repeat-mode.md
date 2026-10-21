---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Repeat in the player, cycling Off, All and One.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product changes playback session repeat.
  kind: product
  entities:
  - entity: playback-session
    effect: changes
    facts:
    - Repeat
  actor: listener
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
---

# Cycle repeat

## Trigger

The listener changes repeat mode.

## Outcome

Off finishes at the context boundary, All wraps the context, and One repeats the current track on natural completion.
