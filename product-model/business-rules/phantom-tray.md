---
appliesTo:
- type: entity
  id: desktop-session
  effect: changes
  facts:
  - Window visibility
references:
- kind: code
  role: implementation
  target: app/src/platform/platform_integration.rs
permits:
- actors:
  - listener
  when:
  - fact: Tray available
    is: true
---

# Phantom window actions require an available tray

The listener can hide and restore Orca through the background integration when its tray is available.
