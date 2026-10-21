---
kind: background
routes:
  desktop: Orca desktop
steps:
- text: Automatic artwork is enabled for this profile.
  kind: condition
  entities:
  - entity: preferences
    effect: reads
    facts:
    - Automatic artwork
  unattended: true
  contexts:
    desktop:
      place: desktop::library
- text: The Product saves matched album and artist covers.
  kind: product
  entities:
  - entity: album
    effect: changes
    facts:
    - Cover
  - entity: artist
    effect: changes
    facts:
    - Cover
  contexts:
    desktop:
      place: desktop::library
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/artwork.rs
- kind: code
  role: implementation
  target: app/src/controllers/editor.rs
- kind: code
  role: implementation
  target: app/src/controllers/collections.rs
- kind: code
  role: implementation
  target: app/src/runtime/events.rs
- kind: code
  role: implementation
  target: app/src/runtime/operations.rs
- kind: code
  role: implementation
  target: app/ui/dialogs.slint
- kind: code
  role: implementation
  target: crates/orca-core/src/online_artwork/cache.rs#read
- kind: code
  role: implementation
  target: app/src/worker/background.rs#spawn
---

# Fetch missing library artwork

## Trigger

Automatic artwork is enabled when the library loads or when the setting is turned on.

## Outcome

Missing album and artist artwork is looked up and saved locally. Existing nonempty custom collection covers are retained.

## Edge cases

- Automatic lookups reuse cached successes and conclusive misses; explicit searches request fresh results.
- Transient failures remain retryable.
- A failure alone does not refresh the catalog.
