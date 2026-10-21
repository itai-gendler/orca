---
appliesTo:
- type: entity
  id: song
  effect: removes
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/scanning.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
permits:
- actors:
  - listener
- unattended: true
---

# The listener and library reconciliation remove indexed songs

Source removal and completed rescans can remove catalog records. These operations do not remove the audio files.
