---
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/catalog.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/media/mod.rs
- kind: code
  role: implementation
  target: crates/orca-core/src/db/songs.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/folder_rules.rs
---

# Song

An indexed local audio file. Exclusion retains its identity and saved associations.

## Information kept

- **Path** — The local audio file location.
- **Title** — The track title.
- **Artist** — The performing artist credit.
- **Album** — The album title.
- **Album artist** — The album credit that distinguishes releases with the same title.
- **Year** — An optional positive year.
- **Track number** — An optional positive track number.
- **Disc number** — An optional positive disc number.
- **Genre** — An optional genre credit.
- **Duration** — The duration of the audio.
- **Quality** — The file format, sample rate and bitrate.
- **Cover** — The embedded artwork and the artwork presented for the track.
- **Lyrics** — Saved lyric text, including any line or word timing.
- **Lyrics removed** — Whether the listener explicitly cleared lyrics, suppressing automatic replacement.
- **Liked** — Whether the track belongs to Liked Songs.
- **Explicit** — The explicit-content advisory read from the file.

## States

### Indexed

Included in the active catalog.

### Excluded

Retained in the profile but hidden by a folder inclusion rule.
