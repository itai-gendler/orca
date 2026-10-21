---
appliesTo:
- type: entity
  id: artist
- type: entity
  id: album
- type: entity
  id: genre
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/collection_names.rs
---

# Bulk renames commit one file at a time

Every member is checked before writing starts. Later failure retains earlier commits and reports partial completion; reopening is required before retrying.
