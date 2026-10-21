---
singleton: true
references:
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/worker/playback.rs
- kind: code
  role: implementation
  target: app/src/service/persistence.rs
---

# Playback session

The current and saved listening session in this profile.

## Information kept

- **Current song** — The selected audio file.
- **Position** — The playback position.
- **Volume** — The output level, including zero for mute.
- **Shuffle** — Whether navigation selects unplayed tracks in shuffled order.
- **Repeat** — Off, All or One.
- **Displayed lyrics** — The lyric text and timing currently available for the playing track.

## States

### Stopped

No audio is playing after startup, completion or loss of the current library track.

### Playing

Audio is advancing.

### Paused

A loaded track is held at its position.
