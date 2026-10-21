---
appliesTo:
- type: entity
  id: song
  effect: changes
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/scanning.rs
permits:
- actors:
  - listener
- unattended: true
---

# The listener and library workflows change song information

The listener edits local music and inclusion choices; scanning and metadata recovery reconcile stored facts. Like actions retain their separate setting condition.
