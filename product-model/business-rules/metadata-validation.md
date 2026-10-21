---
appliesTo:
- type: entity
  id: song
  facts:
  - Title
  - Artist
  - Album
  - Album artist
  - Year
  - Track number
  - Disc number
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs
- kind: code
  role: implementation
  target: app/src/media/metadata.rs
---

# Required tag fields are nonempty and numeric tags are positive

Title, artist, album and album artist are required. Year, track number and disc number are optional positive whole numbers.
