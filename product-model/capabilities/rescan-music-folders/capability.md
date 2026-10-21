---
availability:
- place: desktop
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/scanning.rs#scan_folder
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/service/watcher.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/scanner.rs
domain: library
---

# Rescan music folders

Refresh configured roots manually or after startup and filesystem changes, retaining unavailable sources.
