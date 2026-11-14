# Provenance (licence audit 2026-11-13)

This crate is a transcription of famicom-python (tag `famicom-rust-pin-1`, `fe89099`) and
**inherits famicom-python's verdicts** (its `docs/provenance.md`, "Licence provenance (audit
2026-11-13)"): the same facts, the same citations, the same open items, carried into the
Rust docstrings (`bandai.rs:36` Mesen2's `BandaiFcg.h`; `mmc3_clones.rs:1080` mapper 208's
table "from FCEUX"; `vrc5.rs:27`; the wiki-from-emulator mappers; the MMC3 A12 filter).
Transcribing our own MIT Python into Rust is fine; whatever famicom-python must cite,
re-source or rewrite, this crate must too, in the same places.

The audit read this repository's code comments, docs, `Cargo.toml`/`Cargo.lock` and git
history. No GPL or LGPL source was opened.

## What differs from the reference

| Part | Source | Licence | Verdict |
| --- | --- | --- | --- |
| Crates (`Cargo.lock`): zip, crc32fast, sha1, serde, serde_json, serde-big-array, bincode, flate2 and their dependencies | crates.io | MIT, Apache-2.0, Zlib, Unlicense, 0BSD, Unicode-3.0 (each permissive; checked with `cargo metadata`) | clean |
| `sixfiveohtwo` (6502-rust `71747eb`) | our core, transcribed from 6502-python | MIT | clean (audited with the cores) |
| `ym2413-rust` (the VRC7's sound) | `main`: a path to the restarted ym2413-rust (2026-11-13), not yet pinned by git | the restarted core says it is written from documents | clean (board side); pin it by git when it has a remote |
| `video.rs` | pally's palettes read at run time from the reference's fetched images, never committed | MIT-0 | clean |
| `scripts/`, `probes/`, `scripts/inputs/` | our harness, our probe source (`eval_turn_off.s`), input scripts | | clean |
| No Sunsoft 5B sound | the pin predates famicom-python's 5B work | | n/a |

Counts: clean 5 (plus the inherited verdicts).

## The VRC7 and its history (the repository is public)

- `3cf7a74` put the VRC7's sound on ym2413-rust and `2ffa4e8` pinned it by git at `bdf6365`:
  the old ym2413-rust, a transcription of the old ym2413-python, both withdrawn on
  2026-11-12 as translations of GPL code and made private. Only the dependency line was in
  this tree; before that the board held `OpllPlaceholder`, a silent stand-in with no FM code.
- `e73d73a` switched the sound off (the `vrc7-sound-off` branch, also on GitHub, and merged
  into `origin/main` at `82eb7de`). `6b8536c` switched it back on from the restarted core
  (local `main`, unpushed).
- No commit holds Nuked-derived code.

## Not done here

No SPDX headers yet (0 of 79 source files): owed, as across the family.
