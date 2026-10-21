---
id: orca
summary: A local-first music player for the library you already own.
category: music
tags:
- music
- local-library
- desktop
authors:
- name: Shubham Pathak
  url: https://github.com/shubham-pathak1
license: MIT
languages:
- en
limitations:
- The supported release is Windows x64 and remains in alpha.
- Music playback uses local audio files; music folders supply the library.
- Song tag and embedded-cover edits write to the original audio files.
- Playlists and collection artwork are kept in the local Orca profile.
references:
- kind: doc
  role: intent
  target: README.md
- kind: doc
  role: intent
  target: docs/CONTRIBUTING.md
- kind: code
  role: implementation
  target: app/ui/settings.slint
---

# Orca

Orca is an open-source local music player for listening to, organizing and editing an existing music collection. It combines folder scanning, offline browsing and playback with playlists, lyrics and optional online artwork lookup.

## Intent

“Local-first music player, built for listening to the library you already own.” Orca presents songs, artists, albums, genres, playlists and nested folders as ways to reach that music, with continuity through queues and restored listening sessions.
