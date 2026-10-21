---
kind: edge
routes:
  desktop: Orca desktop
steps:
- text: The playback session needs lyrics for the selected song.
  kind: condition
  entities:
  - entity: playback-session
    effect: reads
    facts:
    - Current song
  - entity: song
    effect: reads
    facts:
    - Lyrics
  unattended: true
  contexts:
    desktop:
      place: desktop::player
- text: The Product reports the unavailable lyric result.
  kind: product
  entities: []
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/lyrics.rs
- kind: code
  role: implementation
  target: app/src/media/lyrics.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
- kind: code
  role: implementation
  target: app/src/worker/background.rs
---

# Display missing or unavailable lyrics

## Trigger

No local lyrics exist and the online request fails or finds no match.

## Outcome

The lyric view reports missing text or a fetch failure and offers retry where applicable.
