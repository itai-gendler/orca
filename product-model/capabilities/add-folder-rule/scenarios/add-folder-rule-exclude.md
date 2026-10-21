---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener turns Include off for a subfolder that follows its included parent.
  kind: actor
  entities:
  - entity: folder
    effect: reads
    facts:
    - Name
  actor: listener
  contexts:
    desktop:
      place: desktop::folder-review
- text: The Product saves an Exclude folder rule for the subfolder and hides its songs from the library.
  kind: product
  entities:
  - entity: folder-rule
    effect: creates
    facts:
    - Path
    - Mode
  - entity: song
    effect: changes
    from: Indexed
    to: Excluded
    facts: []
  actor: listener
  contexts:
    desktop:
      place: desktop::folder-review
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
- kind: code
  role: implementation
  target: app/src/controllers/folders.rs
---

# Exclude a subfolder of an included folder

## Trigger

While managing a music folder, the listener turns off a subfolder they don’t want in the library.

## Outcome

The subfolder’s songs leave browsing and playback; their files, likes and playlist memberships are kept.
