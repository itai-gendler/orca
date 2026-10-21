---
appliesTo:
- type: entity
  id: folder-rule
- type: entity
  id: song
  effect: changes
  to: Excluded
- type: entity
  id: song
  effect: changes
  to: Indexed
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs#included
- kind: code
  role: implementation
  target: crates/orca-services/src/scanning.rs
---

# The nearest folder rule determines inclusion

The longest matching ancestor choice wins; with no matching rule music is included. A nested Include can override an excluded ancestor.
