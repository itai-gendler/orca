---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Use parent for an included subfolder inside an excluded folder.
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
- text: The Product removes the folder rule, and the subfolder’s songs follow their excluded parent out of the library.
  kind: product
  entities:
  - entity: folder-rule
    effect: removes
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

# Return a subfolder to its excluded parent

## Trigger

The listener chooses Use parent for a subfolder whose Include rule overrides an excluded parent.

## Outcome

The subfolder follows its parent again: its songs leave browsing and playback, and their files, likes and playlist memberships are kept.
