---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Unlike song in the player.
  kind: actor
  entities:
  - entity: song
    effect: reads
    facts:
    - Liked
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product clears the playing song’s like.
  kind: product
  entities:
  - entity: song
    effect: changes
    facts:
    - Liked
  actor: listener
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/playlists.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/db/likes.rs
- kind: code
  role: implementation
  target: app/src/controllers/playlists.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
---

# Unlike the playing song

## Trigger

The listener unlikes the song that is playing.

## Outcome

The song leaves Liked Songs; its file and playlists are unchanged.

## Edge cases

- A song no longer active in the library reports an error.
