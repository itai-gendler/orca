---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener turns Include on for a subfolder whose folder rule is Exclude.
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
- text: The Product changes the folder rule to Include and returns the subfolder’s songs to the library.
  kind: product
  entities:
  - entity: folder-rule
    effect: changes
    facts:
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

# Switch a subfolder from Exclude to Include

## Trigger

The listener wants back a subfolder they had explicitly excluded.

## Outcome

Previously excluded songs return with their original identity, likes and playlist memberships, and a scan picks up any new music there.
