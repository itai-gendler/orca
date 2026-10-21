---
references:
- kind: code
  role: implementation
  target: crates/orca-services/src/catalog.rs
- kind: code
  role: implementation
  target: crates/orca-services/src/operations/catalog.rs
---

# Artist

A performing artist collection derived from song credits.

## Information kept

- **Name** — The performing artist credit.
- **Songs** — The matching active tracks.
- **Albums** — Related albums containing those tracks.
- **Cover** — An optional profile artwork choice.
