---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The listener turns Include off for the subfolder holding the track that is playing.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::folder-review
- text: The Product changes the folder rule and excludes the song.
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
- text: The Product prunes the queue and stops the playback session.
  kind: product
  entities:
  - entity: queue
    effect: changes
    facts:
    - Songs
    - Priority songs
    - History
  - entity: playback-session
    effect: changes
    from: Playing
    to: Stopped
    facts:
    - Current song
    - Position
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
  target: app/src/worker/library.rs
---

# Exclude the currently playing track

## Trigger

A changed folder rule excludes the song that is playing.

## Outcome

The song keeps its identity in the profile, but it leaves the queue and playback stops.
