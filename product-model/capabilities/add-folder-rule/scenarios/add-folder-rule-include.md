---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener turns Include on for a subfolder inside an excluded folder.
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
- text: The Product saves an Include folder rule for the subfolder and returns its songs to the library.
  kind: product
  entities:
  - entity: folder-rule
    effect: creates
    facts:
    - Path
    - Mode
  - entity: song
    effect: changes
    from: Excluded
    to: Indexed
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

# Include a subfolder inside an excluded folder

## Trigger

A subfolder of an excluded folder holds music the listener wants back.

## Outcome

Previously excluded songs return with their original identity, likes and playlist memberships, and a scan picks up any new music there.
