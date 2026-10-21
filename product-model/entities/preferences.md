---
singleton: true
references:
- kind: code
  role: implementation
  target: app/src/controllers/preferences.rs
- kind: code
  role: implementation
  target: app/ui/settings.slint
- kind: code
  role: implementation
  target: app/src/protocol/mod.rs
---

# Preferences

The profile settings reused across launches.

## Information kept

- **Liked Songs cover** — The custom artwork for the built-in liked selection.
- **Appearance** — The theme, font, font size and visuals: where artwork, cover colors and backgrounds appear.
- **Interface** — The sidebar mode and the seekbar style.
- **Show Liked Songs** — Whether the Liked Songs selection and player like controls are shown.
- **Automatic artwork** — Whether missing album and artist artwork is fetched automatically.
- **Gapless playback** — Whether the next eligible track is preloaded.
- **ReplayGain** — Whether tagged loudness adjustment is used.
- **Quality info** — Whether track audio quality information is shown.
