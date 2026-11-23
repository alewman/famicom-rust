# Provenance (licence audit 2026-11-13)

This crate is a transcription of famicom-python (tag `famicom-rust-pin-1`, `fe89099`, now
gated against `e3e1474`) and **inherits famicom-python's verdicts** (its
`docs/provenance.md`): the same facts, the same citations, the same open items, carried into
the Rust docstrings. Transcribing our own MIT Python into Rust is fine; whatever
famicom-python must cite, re-source or rewrite, this crate must too, in the same places.

**The rewording of 2026-11-14 to 2026-11-19 carried (2026-11-23).** famicom-python's
provenance wave 2 (`81f8a9d`, `4320ef4`, `c23aa6c`) re-established the facts its docstrings
had cited from Mesen2's source, as DOC, OBSERVED closed-box (Mesen2 2.1.1 run as an
instrument by its `scripts/provenance_probes.py`), or MAME's BSD-3 drivers, and dropped the
MMC3 A12 filter's furrtek-hosted schematic for the test ROMs. The Rust docstrings that
carried the old wording now carry the new: `bandai.rs` (the FCG counter's test-then-decrement
order), `fds.rs` (`spin_up_bytes`), `kaiser.rs` (mapper 43's file layout, now the wiki's),
`mmc3.rs` (`A12_LOW_FALLS`, TEST ROM), `mmc3_clones.rs` (mapper 45's power-on, 165's latch
ranges, 208's table, measured closed-box and checked against the game's own jumps by
`tests/mappers_since_pin_1.rs`) and `namco.rs` (the 163's IRQ counting only while enabled).
`discrete.rs` (185's power-on) and `mmc5.rs` (ExRAM writes outside rendering) never carried
the old citation. No source line names Mesen2's source files now; `vrc5.rs` and the
wiki-from-emulator mappers keep the reference's wording, unchanged there.

The audit read this repository's code comments, docs, `Cargo.toml`/`Cargo.lock` and git
history. No GPL or LGPL source was opened.

## What differs from the reference

| Part | Source | Licence | Verdict |
| --- | --- | --- | --- |
| Crates (`Cargo.lock`): zip, crc32fast, sha1, serde, serde_json, serde-big-array, bincode, flate2 and their dependencies | crates.io | MIT, Apache-2.0, Zlib, Unlicense, 0BSD, Unicode-3.0 (each permissive; checked with `cargo metadata`) | clean |
| `sixfiveohtwo` (6502-rust `71747eb`) | our core, transcribed from 6502-python | MIT | clean (audited with the cores) |
| `ym2413-rust` (the VRC7's sound) | the restarted ym2413-rust, pinned by git at `94df223` | the restarted core says it is written from documents | clean (board side) |
| `ay8910-rust` (the Sunsoft 5B's sound, 2026-11-23) | pinned by git at `9b670a7`, a transcription of ay8910-python `a3bb485`, whose four GPL-read behaviours were re-derived closed-box before its history reset | MIT | clean (audited with the cores) |
| `video.rs` | pally's palettes read at run time from the reference's fetched images, never committed | MIT-0 | clean |
| `scripts/`, `probes/`, `scripts/inputs/` | our harness, our probe source (`eval_turn_off.s`), input scripts | | clean |
| `scripts/cases-5b.tsv` | rainwarrior's nes-audio-tests, run from the reference's fetched `reference/`, never committed (licence: "may be freely redistributed and modified for any purpose") | permissive | clean |

Counts: clean 7 (plus the inherited verdicts).

## The VRC7 and its history (the repository is public)

- `3cf7a74` put the VRC7's sound on ym2413-rust and `2ffa4e8` pinned it by git at `bdf6365`:
  the old ym2413-rust, a transcription of the old ym2413-python, both withdrawn on
  2026-11-12 as translations of GPL code and made private. Only the dependency line was in
  this tree; before that the board held `OpllPlaceholder`, a silent stand-in with no FM code.
- `e73d73a` switched the sound off (the `vrc7-sound-off` branch, also on GitHub, and merged
  into `origin/main` at `82eb7de`). `6b8536c` switched it back on from the restarted core
  (local `main`, unpushed).
- No commit holds Nuked-derived code.

## Headers

Every code, test, script and CI file carries `SPDX-License-Identifier: MIT` (`2bc494a`;
the files added since carry it too).
