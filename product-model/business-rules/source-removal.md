---
appliesTo:
- type: entity
  id: music-folder
  effect: removes
- type: entity
  id: song
  effect: removes
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
---

# Removing a music root preserves audio files

Source removal deletes only catalog records no remaining configured root covers, along with their saved associations. Music files are preserved.
