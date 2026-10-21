---
kind: primary
routes:
  desktop: Orca desktop
steps:
- text: The listener chooses Refresh lyrics for the playing track.
  kind: actor
  entities: []
  actor: listener
  contexts:
    desktop:
      place: desktop::player
- text: The Product checks song identity and refreshes playback session lyrics.
  kind: product
  entities:
  - entity: song
    effect: reads
    facts:
    - Title
    - Artist
    - Album
    - Duration
    - Lyrics removed
  - entity: playback-session
    effect: changes
    facts:
    - Displayed lyrics
  actor: listener
  contexts:
    desktop:
      place: desktop::player
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/media/lyrics.rs
- kind: code
  role: implementation
  target: app/src/worker/library.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
- kind: code
  role: implementation
  target: app/ui/players.slint
---

# Refresh current listening lyrics

## Trigger

The listener chooses Refresh lyrics for the current track.

## Outcome

Fresh text replaces displayed lyrics for the current listening session, leaving saved lyrics and the audio file unchanged.

## Edge cases

- An explicit prior lyric removal is respected; the metadata editor offers explicit fetching.
- Newly saved local lyrics during a request take precedence over its older result.
