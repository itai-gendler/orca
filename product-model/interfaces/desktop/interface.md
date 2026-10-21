---
type: desktop-app
actors:
- listener
languages:
- en
references:
- kind: code
  role: implementation
  target: app/src/main.rs
- kind: code
  role: implementation
  target: app/src/platform/media_session.rs
- kind: code
  role: implementation
  target: app/src/platform/platform_integration.rs
---

# Orca desktop

The local Windows application, including its keyboard commands, tray actions and Windows media controls. The listener operates the selected local profile without an Orca account or sign-in. Provider requests are outbound parts of artwork and lyrics abilities.
