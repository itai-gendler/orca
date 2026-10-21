---
entities:
- entity: queue
  shows:
  - Songs
  - Priority songs
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
- entity: playback-session
  shows:
  - Current song
  - Shuffle
  - Repeat
references:
- kind: code
  role: implementation
  target: app/src/playback/navigation.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
---

# Queue

The current track and a preview of upcoming listening choices.
