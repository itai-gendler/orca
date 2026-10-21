---
singleton: true
domain: queue
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
---

# Queue

The current listening order, including explicit additions and navigation history.

## Information kept

- **Songs** — The ordered playback context and its removals.
- **Priority songs** — Explicit additions consumed before the remaining context.
- **History** — Previously played tracks used when retracing shuffle navigation.
