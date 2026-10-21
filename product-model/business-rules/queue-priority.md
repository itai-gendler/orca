---
appliesTo:
- type: entity
  id: queue
- type: entity
  id: playback-session
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs#next
---

# Explicit queue additions precede the remaining collection

Next and natural endings consume explicit additions before other tracks, except natural Repeat One. After external additions are consumed, navigation returns to the remaining original context.
