---
appliesTo:
- type: capability
  id: edit-song-metadata
- type: journey
  id: save-found-cover
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/library/metadata.rs
---

# Saved metadata reflects the file that committed

Song saves check the source version, write and verify tags, then reconcile the index. Interrupted reconciliation rereads the committed file.
