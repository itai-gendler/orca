---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Like song in the player.
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
- text: The Product marks the playing song as liked.
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

# Like the playing song

## Trigger

The listener likes the song that is playing.

## Outcome

The song appears first in Liked Songs, which lists the newest likes first.

## Edge cases

- A song no longer active in the library reports an error.
