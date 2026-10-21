---
appliesTo:
- type: entity
  id: song
  facts:
  - Lyrics
  - Lyrics removed
- type: entity
  id: playback-session
  facts:
  - Displayed lyrics
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/lyrics.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/media.rs
---

# Explicit lyric removal precedes local and automatic lyrics

Without explicit removal, sidecar LRC text precedes cached and embedded text. Automatic missing-lyric loading uses saved or session text before fetching. The metadata editor can explicitly fetch new lyrics after removal.
