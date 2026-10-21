---
availability:
- place: desktop
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
- kind: code
  role: implementation
  target: app/src/controllers/folders.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
domain: library
---

# Add music folder

Register a local root, review subfolders when present, and start scanning the included music.
