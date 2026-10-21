---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Use parent for an excluded subfolder inside an included folder.
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
- text: The Product removes the folder rule, and the subfolder’s songs follow their included parent back into the library.
  kind: product
  entities:
  - entity: folder-rule
    effect: removes
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

# Return a subfolder to its included parent

## Trigger

The listener chooses Use parent for a subfolder whose Exclude rule overrides an included parent.

## Outcome

The subfolder follows its parent again: its songs return with their original identity, likes and playlist memberships.
