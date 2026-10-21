---
singleton: true
references:
- kind: code
  role: implementation
  target: app/src/platform/platform_integration.rs
- kind: code
  role: implementation
  target: app/src/main.rs
---

# Desktop session

The current Orca window and its background playback integration.

## Information kept

- **Window visibility** — Visible or hidden in Phantom Mode.
- **Tray available** — Whether the tray integration initialized successfully.
