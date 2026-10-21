---
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/folders.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
---

# Folder

A directory reached while browsing music or reviewing a root; its name comes from the filesystem.

## Information kept

- **Path** — The directory location.
- **Name** — The directory name.
- **Songs** — Indexed music beneath this directory.
- **Subfolders** — Child directories available in the current browsing or review context.
- **Cover** — An optional profile artwork choice.
