---
appliesTo:
- type: entity
  id: playback-session
- type: entity
  id: queue
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
---

# Playback stays within its listening context

Repeat Off stops at the end unless priority additions remain. All wraps the context, including a single shuffled track. One repeats the current track on natural completion, while explicit Next can leave it.
