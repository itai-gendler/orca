---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener drags an upcoming track higher in Queue, or presses Alt+Up on it.
  kind: actor
  entities:
  - entity: queue
    effect: reads
    facts:
    - Songs
  actor: listener
  contexts:
    desktop:
      place: desktop::queue
- text: The Product places the track before the one it was moved above, keeping the current track in place.
  kind: product
  entities:
  - entity: queue
    effect: changes
    facts:
    - Songs
    - Priority songs
  actor: listener
  contexts:
    desktop:
      place: desktop::queue
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
---

# Move an upcoming track earlier

## Trigger

The listener wants a queued track to play sooner.

## Outcome

Queue shows the track at its new place, and Next follows the new order.

## Edge cases

- The current track is never moved.
