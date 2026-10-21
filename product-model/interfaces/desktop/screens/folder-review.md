---
entities:
- entity: music-folder
  shows:
  - Path
- entity: folder
  shows:
  - Path
  - Name
  - Subfolders
- entity: folder-rule
  shows:
  - Path
  - Mode
  collects:
  - Path
  - Mode
references:
- kind: code
  role: implementation
  target: app/ui/folder_review.slint
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
---

# Music folder review

The selected music root and its subfolder inclusion choices.
