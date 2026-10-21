---
domain: library
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
---

# Folder rule

An explicit inclusion choice on a subfolder of a configured music root.

## Information kept

- **Path** — The reviewed subfolder.
- **Mode** — Include or Exclude; no rule means inherit the nearest ancestor choice.
