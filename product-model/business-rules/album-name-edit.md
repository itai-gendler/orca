---
appliesTo:
- type: entity
  id: album
  effect: changes
  facts:
  - Title
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
permits:
- actors:
  - listener
---

# The listener edits album names

A local collection edit writes the release title across its active tracks.
