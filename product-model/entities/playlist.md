---
domain: playlists
relations:
- entity: song
  verb: includes
  cardinality: many-to-many
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/playlists.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/db/playlists.rs
---

# Playlist

A named, locally saved selection of songs. Liked Songs is a built-in selection derived from likes, rather than a user-created playlist.

## Information kept

- **Name** — The listener-supplied name.
- **Songs** — The ordered, unique song memberships, including retained excluded songs.
- **Cover** — An optional custom cover; active member artwork supplies a fallback.
