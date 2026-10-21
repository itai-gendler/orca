---
appliesTo:
- type: entity
  id: artist
  effect: changes
  facts:
  - Name
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
permits:
- actors:
  - listener
---

# The listener edits artist names

A local collection edit writes the performing credit across its active tracks.
