---
entities:
- entity: playback-session
  shows:
  - Current song
  - Position
  - Volume
  - Shuffle
  - Repeat
  - Displayed lyrics
- entity: song
  shows:
  - Title
  - Artist
  - Album
  - Album artist
  - Duration
  - Quality
  - Cover
  - Explicit
  - Liked
references:
- kind: code
  role: implementation
  target: app/ui/players.slint
- kind: code
  role: implementation
  target: app/src/platform/media_session.rs
---

# Player

The current listening session, in the compact or the full player, with its lyrics, queue and playlist controls.
