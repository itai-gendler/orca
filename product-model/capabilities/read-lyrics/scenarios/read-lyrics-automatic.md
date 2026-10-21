---
kind: background
routes:
  desktop: Orca desktop
steps:
- text: The playback session has changed its current track.
  kind: condition
  entities:
  - entity: playback-session
    effect: reads
    facts:
    - Current song
  unattended: true
  contexts:
    desktop:
      place: desktop::player
- text: The Product selects song lyrics and makes text available in the playback session.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Path
    - Lyrics
    - Lyrics removed
  - entity: playback-session
    effect: changes
    facts:
    - Displayed lyrics
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

# Load local lyrics or fetch missing text

## Trigger

The current track changes.

## Outcome

Local lyric sources are preferred. If no saved or session text exists and lyrics were not deliberately removed, an online result becomes available for this listening session.

## Edge cases

- A same-name LRC sidecar precedes cached and embedded lyrics unless explicit removal suppresses them.
- Automatic online results do not write the audio file.
- A transient failure leaves local browsing and listening available.
