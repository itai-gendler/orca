---
availability:
- place: desktop
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/artwork.rs
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
- kind: code
  role: implementation
  target: app/src/controllers/collections.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
- kind: code
  role: implementation
  target: app/src/runtime/operations.rs
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
---

# Find cover online

Search for album artwork or an artist portrait. A track action opens its metadata editor; editor results wait for Save, while a direct collection action applies profile artwork.
