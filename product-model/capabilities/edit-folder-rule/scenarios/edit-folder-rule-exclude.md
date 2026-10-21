---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener turns Include off for a subfolder whose folder rule is Include.
  kind: actor
  entities:
  - entity: folder
    effect: reads
    facts:
    - Name
  - entity: folder-rule
    effect: reads
    facts:
    - Mode
  actor: listener
  contexts:
    desktop:
      place: desktop::folder-review
- text: The Product changes the folder rule to Exclude and hides the subfolder’s songs from the library.
  kind: product
  entities:
  - entity: folder-rule
    effect: changes
    facts:
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

# Switch a subfolder from Include to Exclude

## Trigger

The listener no longer wants a subfolder they had explicitly included.

## Outcome

The subfolder’s songs leave browsing and playback; their files, likes and playlist memberships are kept.
