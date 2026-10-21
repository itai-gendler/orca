---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener clears the lyric text and saves.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
- text: The Product changes song lyrics and records explicit removal.
  kind: product
  entities:
  - entity: song
    effect: changes
    facts:
    - Lyrics
    - Lyrics removed
  actor: listener
  contexts:
    desktop:
      place: desktop::metadata-editor
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/media/lyrics.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/library/metadata.rs
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
---

# Save deliberate lyric removal

## Trigger

The listener clears existing lyrics and saves.

## Outcome

Embedded lyrics are cleared and automatic loading respects the removal, including when an older sidecar remains.
