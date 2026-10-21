---
entities:
- entity: music-folder
  shows:
  - Path
  - Available
  - Scan progress
  collects:
  - Path
- entity: preferences
  shows:
  - Automatic artwork
  collects:
  - Automatic artwork
references:
- kind: code
  role: implementation
  target: app/ui/settings.slint
- kind: code
  role: implementation
  target: app/src/worker/library.rs
---

# Library

Configured music folders, scan actions and automatic artwork settings.
