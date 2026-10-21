---
scope: The Windows desktop music library, listening, editing, collections and profile preferences.
method: Static tracing of UI callbacks, application workflows, persistence and focused test source at the pinned
  commit.
covered:
- description: Desktop catalog, settings, editors and player interaction code.
  paths:
  - app/ui/
  - app/src/controllers/
  - app/src/views/
  - app/src/runtime/
  - app/src/main.rs
- description: Scanning, folder reconciliation and catalog workflow code.
  paths:
  - app/src/worker/
  - app/src/service/
  - crates/orca-services/src/scanning.rs
  - crates/orca-services/src/folders.rs
  - crates/orca-services/src/folder_rules.rs
  - crates/orca-services/src/catalog.rs
  - crates/orca-core/src/scanner.rs
  - crates/orca-core/src/db/
- description: Playback navigation, audio command and media integration code.
  paths:
  - app/src/playback/
  - app/src/platform/
  - crates/orca-services/src/playback.rs
  - crates/orca-core/src/audio_engine/
- description: Metadata, artwork, lyric and playlist workflow code.
  paths:
  - app/src/media/
  - crates/orca-services/src/media/
  - crates/orca-services/src/operations/
  - crates/orca-core/src/library/
exclusions:
- description: Build, packaging and CI scripts are contributor tooling rather than modeled listening behavior.
  paths:
  - scripts/
  - .github/
  - app/build.rs
- description: Design assets, font files and rendering geometry are presentation implementation.
  paths:
  - app/ui/assets/
  - app/ui/icons/
  - app/src/media/lyric_render/
- description: Transport types, atomic file helpers and job scheduling are implementation mechanisms.
  paths:
  - app/src/protocol/
  - crates/orca-services/src/types/
  - crates/orca-services/src/operation_types.rs
  - crates/orca-core/src/atomic_file.rs
- description: Test harnesses and fixture construction provide evidence rather than additional product contracts.
  paths:
  - app/src/tests/
  - crates/orca-services/src/tests/
  - crates/orca-core/tests/
  - crates/orca-core/examples/
unmapped:
- description: Legacy database import, schema-version upgrade and artwork localization compatibility are deferred
    release-maintenance workflows.
  paths:
  - crates/orca-services/src/bootstrap.rs
  - crates/orca-services/src/bootstrap/
  - crates/orca-core/src/db/schema.rs
- description: Provider-specific artwork ranking and lyric matching algorithms are deferred beneath the mapped lookup
    outcomes.
  paths:
  - crates/orca-core/src/online_artwork/
  - crates/orca-core/src/online_artwork.rs
  - crates/orca-core/src/lyrics.rs
- description: Audio device negotiation and low-level equalizer or crossfade APIs lack a traced desktop settings
    contract and are deferred.
  paths:
  - crates/orca-core/src/audio_output/
  - crates/orca-core/src/audio_output.rs
  - crates/orca-core/src/audio_engine/sources.rs
- description: Scrobbling client code has no current desktop entry point.
  paths:
  - crates/orca-core/src/lastfm.rs
limitations: []
---

# Coverage


