---
appliesTo:
- type: entity
  id: folder
  effect: changes
  facts:
  - Name
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
permits: []
---

# Folder names are changed outside Orca

Orca presents directory names from the filesystem. Its collection editor changes their covers only.
