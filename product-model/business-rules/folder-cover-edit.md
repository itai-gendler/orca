---
appliesTo:
- type: entity
  id: folder
  effect: changes
  facts:
  - Cover
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
permits:
- actors:
  - listener
---

# The listener changes folder covers

The selected directory can have profile artwork without changing its filesystem name.
