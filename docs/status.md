<!-- SPDX-License-Identifier: MIT -->
# Status

The transcription tier of the NES line: famicom-python (the specification) →
**famicom-rust** → famicom-rust-fast. The reference is famicom-python `b5dbd2f`
(`scripts/fetch_famicompython.sh`; its `src/` is `c52a6d3`'s). The README's
certification table says what each rung covers; this page is the gate and its latest
counts.

## The gate: `scripts/ladder.sh`

Measured on 2026-11-24 on the tree of the commit that adds the watchdog case (the
PlayChoice-10's `pc10-watchdog-Super_Mario_Bros`, `--bios-program`), with the
reference traces cached in `external/` (made by famicom-python at `b3dbf1b`, the new
case's at `b5dbd2f`; nothing under the reference's `src/` changed between them, see
the README), `JOBS=6` on a shared machine at load 100 (6,340 s,
`external/ladder-watchdog.log`). The ladder ran on a build of `read_program` before a
clippy rewording of it (an index loop made a slice copy); the rebuilt binary was run
again on `cargo test --test playchoice` (27 of 27) and the PlayChoice-10 table.

| Step | Count |
| --- | --- |
| `cargo test --release` | all pass; `tests/playchoice.rs` 27 (the real-BIOS cases with `MAME_ROMPATH` and `PC10_IMAGES_DIR`), `tests/vs_system.rs` 32 |
| Rung 1, nestest | 8,991 lines, 0 differ; result codes 00 00 |
| Rung 2, lockstep | 200 of 200; the probes 2 of 2 |
| Rung 5, lockstep | every board class 581 of 581 (600 slices, `external/cases-rung5.tsv`); the FDS 6 of 6; Lagrange Point 1 of 1; AutoDisk 3 of 3; the 5B 9 of 9; mapper 559 2 of 2 |
| Rung 6, lockstep | machine rows 33 of 33; peripherals 54 of 54; host input 36 of 36; the Vs. System 66 of 66; the PlayChoice-10 61 of 61 (the watchdog case: 54 watchdog resets in 600 slices) |
| Rung 3, test ROM tables | 197 of 197 agree |
| Rung 4, AccuracyCoin | 147 of 147 on the reference's 126,984,584 CPU cycles and 4,264 frames |
| Save states | 363 of 363 dumps; the 5B and 559, 5 of 5; Vs. UniSystem images 43 of 43; PlayChoice-10 cabinets 56 of 56 |
| CI's checks | `cargo clippy --release --all-targets -- -D warnings` and `cargo fmt --check` clean with the stable toolchain (rustc 1.99.1) |

## Open

- Seeded faults are famicom-rust-fast's (`scripts/seeded_faults.py`, run there on the
  shared code, the PlayChoice-10's included); this tier has no fault script of its own.
- No save state is taken while the watchdog counts: `famicom-pc10-states` runs the
  real BIOS, which never lets it (a `--bios-program` there would close it).
- Not carried: the reference window's Vs. and PlayChoice-10 inputs and screens
  (`frontend.py`).
- The PlayChoice-10 is the reference's first pass, uncertified there: the traces match
  the specification, which says nothing yet about the hardware (famicom-python
  docs/validation.md, "The PlayChoice-10"). The watchdog case is a program of our own,
  not the BIOS: it proves the two tiers agree through the Z80's reset, nothing more.
