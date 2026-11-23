# famicom-rust

A Rust Famicom / NES board transcribed from
[famicom-python](https://github.com/alewman/famicom-python) (transcribed at
`fe89099`, the old tag `famicom-rust-pin-1`; gated since 2026-11-23 against
`e3e1474`, see below) on [6502-rust](https://github.com/alewman/6502-rust)'s
RP2A03, and proven equivalent to it by a lockstep gate: both boards run the
same cartridge and their traces are compared frame by frame, every CPU bus
cycle (address, value, direction and master-clock tick), the picture, the
PPU's v/t/status and the audio samples to the bit.

The board is the reference's, module for module: one bus call per CPU cycle
with OAM and DMC DMA inside the stalled read, open bus, the dot-based 2C02
with every fetch on the cartridge connector, the APU with its mixer and the
consoles' fitted analogue stages, the machine rows (NES-001, Famicom, AV
Famicom, NES-101, PAL NES, Dendy), the iNES / NES 2.0 loader with nes20db, the
reference's mappers, the Famicom Disk System (drive and sound) and the
expansion sound chips. Names and hardware comments' sources are carried over;
where Rust forced a difference, the module header says so
(`src/cartridge/connector.rs` explains how the reference's class hierarchy
became traits).

```text
cargo build --release
target/release/famicom ROM.zip --frames 600 --png last.png --wav sound.wav
```

It runs at about eight to ten times real time (Super Mario Bros.: 300 frames in 0.5 s),
some 25 times the reference on PyPy: correct, not yet fast. Speed is the job
of a `-fast` fork.

## Certification

| What | Version |
| --- | --- |
| famicom-python (the reference) | `e3e1474` (its main of 2026-11-23), pinned by commit in `scripts/fetch_famicompython.sh` |
| 6502-rust | `71747eb` (its NMOS core is 6502-python `9dc34bd`'s; the reference pins `e73f471`, the same tree after a history rewrite) |
| ym2413-rust (the VRC7's FM) | `94df223`, the restarted core, written from documents; the reference pins ym2413-python `67fdb1e` |
| ay8910-rust (the Sunsoft 5B's sound) | `9b670a7`, a transcription of ay8910-python `a3bb485`; the reference pins `e9420aa`, whose `src/` is the same but for the SPDX lines `a3bb485` added (compared file by file against the reference's install, 2026-11-23) |
| nes-test-roms | christopherpow `95d8f62` (the reference's pin) |
| nes20db | 2026-09-26 (the reference's pin, read at run time, never committed) |

**The repin of 2026-11-23.** The reference was the tag `famicom-rust-pin-1`
(`47d33c1`, `fe89099` before famicom-python's history rewrite), whose pyproject
pinned the archived ym2413-python `13df60c`, a translation of GPL code. It is now
famicom-python `e3e1474`, whose cores are all public pins (6502-python `e73f471`,
ym2413-python `67fdb1e`, ay8910-python `e9420aa`; z80-python 0.4.0). Every rung
below was run again against it, the reference caches made afresh: all identical
(the regenerated reference traces are byte-for-byte the old caches', 902 of 902;
rung 3's outcomes differ only by two new header fields, `vs_ppu` and
`vs_hardware`).

Carried since (2026-11-23): the Sunsoft 5B's sound (mapper 69 on ay8910-rust,
with cases that program it: rainwarrior's nes-audio-tests and Gimmick!), mapper
559, and the reference's provenance rewording of the mapper docstrings. The
board does **not** yet carry, and no case here exercises: the Vs. System
(mapper 99, `vs.py`, `dual.py`, the RP2C03/RP2C04/RC2C05 rows, the Namco 108
protection chips) and the PlayChoice-10 (`playchoice.py`, `supervisor.py` on
z80-python 0.4.0, which no Rust Z80 transcribes: z80-rust is z80-python
`b59c80b`, after 0.4.0's EI-delay and reset fixes).

The ladder is the brief's (`docs/` of the handoff), in its order, and then save states:

| Rung | What | Result | Script |
| --- | --- | --- | --- |
| 1 | nestest in automation mode against nestest.log: PC, registers, P, SP, CPU cycle, PPU line and dot per instruction | 8,991 of 8,991 lines; result codes 00 00 | `rung1.sh` |
| 2 | The lockstep gate on every NROM test ROM the reference carries (600 slices) and AccuracyCoin's run-all (3,000); then the probes, our own test ROMs (`probes/*.s`, assembled with cc65 by `probes/build.sh`): `eval_turn_off.s` turns rendering off in the middle of sprite evaluation at a different dot each frame (329 times in 600 frames, at dots 66-255, both parities) and reads back OAM at the address evaluation left | 200 of 200 and 1 of 1 identical | `rung2.sh` |
| 3 | The reference's test ROM tables (its `pytest -m slow` cases: CPU, PPU, APU, mappers, FDS, PAL) through a transcription of its harness: verdict, CPU cycles, instructions and detail text against the reference's own outcomes | 197 of 197 agree; the reference passes 197 | `rung3.sh` |
| 4 | AccuracyCoin, all tests, as the reference's `scripts/accuracycoin.py` runs them | 147 of 147 pass, on the reference's 126,984,584 CPU cycles and 4,264 frames (4.7 s against its 315 s) | `rung4.sh` |
| 5 | Every mapper the reference builds but 99, the Vs. System's (295 of its 296; `tests/mapper_coverage.rs`), in lockstep on dumps that use it: up to two dumps of every board class the reference builds for the local dumps (scripts/mapper_dumps.py indexes them), 600 slices with Start pressed now and then; the FDS on three disks, NES and Famicom; Lagrange Point's VRC7 FM music (ym2413-rust) on the Famicom, 3,000 slices; AutoDisk (`fds_auto.rs`, the reference's `fds_auto.py`, the sides changed as the program asks) on two disks that ask for side B and one that waits for a disk the dump lacks; the Sunsoft 5B's sound, `scripts/cases-5b.tsv`: rainwarrior's six 5B ROMs (`db_5b`, `clip_5b`, `envelope_5b`, `noise_5b`, `phase_5b`, `sweep_5b`) and *Gimmick!* (Japan), 3,000 slices, on the Famicom, which routes the cartridge's sound, and two on the NES-001; mapper 559 on its one dump, `scripts/cases-559.tsv` | 581 of 581, 6 of 6, 1 of 1, 3 of 3, 9 of 9 and 2 of 2 identical | `rung5.sh` |
| 6 | The machine rows (PAL and Dendy on ten test ROMs; AccuracyCoin's run-all on PAL, Dendy, Famicom, AV Famicom, NES-101) and the peripherals in lockstep | machine rows 25 of 25 identical; peripherals 54 of 54 identical (every device class of `peripherals.py`, driven by scripted inputs: Zapper, Vaus, Power Pad, keyboards and data recorder, mice, R.O.B. ...); the host input (`host.rs`, the input half of the reference's window `frontend.py`: its device choice, players and `_apply_inputs`, on a stand-in pygame) driving every device from one player script, the window's settings written beside each frame: 36 of 36 identical | `rung6.sh` |
| — | Save states (`state.rs`, from famicom-python `c497645`'s `state.py`): saved at frame 60 plus a random number of cycles, loaded into a fresh system and into one that had run elsewhere, each continues as the run that was not interrupted (bus cycles, picture, audio, registers, master clock). Format version 2 since 2026-11-23 (the VRC7's restarted chip, the 5B): a state of another version is refused before it is decoded, and a state that fails to load leaves the console as it was (`tests/state_failed_load.rs`) | 363 of 363 dumps, one of every board class; the 5B on the Famicom (*Gimmick!*, four of the 5B ROMs) and mapper 559, 6 of 6 | `famicom-state-roundtrip` |

A slice is one PPU frame (`System::run_frames(1)`); the trace line holds the
CPU registers, cycle and instruction counts, a CRC of every bus cycle, of the
picture and of the slice's audio. `famicom-trace ... --detail N` and
`scripts/famicom_trace.py ... --detail N` print every bus cycle of slice N.

The reference and the ROMs are local, so rungs 2-6 run by hand:

```text
LOCAL=/path/to/famicom-python scripts/fetch_famicompython.sh   # the pin, a PyPy venv, links to its test ROMs and references
scripts/rung1.sh && scripts/rung2.sh && scripts/rung3.sh && scripts/rung4.sh
external/famicom-python/.venv-pypy/bin/python scripts/mapper_dumps.py /path/to/nes-dumps --out external/mapper-dumps.tsv
scripts/rung5.sh && scripts/rung6.sh
```

The FDS needs Nintendo's BIOS, never shipped: `FAMICOM_FDS_BIOS` names it.
CI runs the build, clippy, the tests and rung 1.

## Licence

MIT, as the reference.
