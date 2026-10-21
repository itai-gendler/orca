---
domain: library
relations:
- entity: folder-rule
  verb: has
  cardinality: one-to-many
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/scanning.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/mutations.rs
---

# Music folder

A configured root that Orca scans for music, including eligible subfolders.

## Information kept

- **Path** — The configured root directory.
- **Available** — Whether the root can currently be reached.
- **Scan progress** — Whether scanning is active and how many supported files have been considered.
