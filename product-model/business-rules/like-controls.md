---
appliesTo:
- type: entity
  id: song
  effect: changes
  facts:
  - Liked
references:
- kind: code
  role: implementation
  target: app/ui/settings.slint
- kind: code
  role: implementation
  target: app/ui/players.slint
permits:
- actors:
  - listener
  when:
  - entity: preferences
    fact: Show Liked Songs
    is: true
---

# The listener changes likes when Liked Songs is shown

The Interface setting controls access to player like actions and the built-in selection. Existing likes remain when hidden.
