---
appliesTo:
- type: entity
  id: album
  effect: changes
  facts:
  - Cover
- type: entity
  id: artist
  effect: changes
  facts:
  - Cover
references:
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/artwork.rs
permits:
- actors:
  - listener
- unattended: true
  when:
  - entity: preferences
    fact: Automatic artwork
    is: true
---

# Automatic cover changes require the artwork setting

The listener can request artwork changes. Unattended cover changes run only while automatic artwork is enabled.
