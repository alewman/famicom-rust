# famicom-rust

A Rust Famicom / NES board transcribed from
[famicom-python](https://github.com/alewman/famicom-python) (frozen at the tag
`famicom-rust-pin-1`, `fe89099`) on [6502-rust](https://github.com/alewman/6502-rust)'s
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
| famicom-python (the reference) | tag `famicom-rust-pin-1` (`fe89099`) |
| 6502-rust | `71747eb` (its NMOS core is 6502-python `9dc34bd`'s, the revision the reference pins) |
| nes-test-roms | christopherpow `95d8f62` (the reference's pin) |
| nes20db | 2026-09-26 (the reference's pin, read at run time, never committed) |

The ladder is the brief's (`docs/` of the handoff), in its order, and then save states:

| Rung | What | Result | Script |
| --- | --- | --- | --- |
| 1 | nestest in automation mode against nestest.log: PC, registers, P, SP, CPU cycle, PPU line and dot per instruction | 8,991 of 8,991 lines; result codes 00 00 | `rung1.sh` |
| 2 | The lockstep gate on every NROM test ROM the reference carries (600 slices) and AccuracyCoin's run-all (3,000) | 200 of 200 identical | `rung2.sh` |
| 3 | The reference's test ROM tables (its `pytest -m slow` cases: CPU, PPU, APU, mappers, FDS, PAL) through a transcription of its harness: verdict, CPU cycles, instructions and detail text against the reference's own outcomes | 197 of 197 agree; the reference passes 197 | `rung3.sh` |
| 4 | AccuracyCoin, all tests, as the reference's `scripts/accuracycoin.py` runs them | 147 of 147 pass, on the reference's 126,984,584 CPU cycles and 4,264 frames (4.7 s against its 315 s) | `rung4.sh` |
| 5 | Every mapper the reference builds (all 294; `tests/mapper_coverage.rs`), in lockstep on dumps that use it: up to two dumps of every board class the reference builds for the local dumps (scripts/mapper_dumps.py indexes them), 600 slices with Start pressed now and then; the FDS on three disks, NES and Famicom; Lagrange Point's VRC7 FM music (ym2413-rust) on the Famicom, 3,000 slices; AutoDisk (`fds_auto.rs`, the reference's `fds_auto.py`, the sides changed as the program asks) on two disks that ask for side B and one that waits for a disk the dump lacks | 581 of 581, 6 of 6, 1 of 1 and 3 of 3 identical | `rung5.sh` |
| 6 | The machine rows (PAL and Dendy on ten test ROMs; AccuracyCoin's run-all on PAL, Dendy, Famicom, AV Famicom, NES-101) and the peripherals in lockstep | machine rows 25 of 25 identical; peripherals 54 of 54 identical (every device class of `peripherals.py`, driven by scripted inputs: Zapper, Vaus, Power Pad, keyboards and data recorder, mice, R.O.B. ...) | `rung6.sh` |
| — | Save states (`state.rs`, from famicom-python `c497645`'s `state.py`): saved at frame 60 plus a random number of cycles, loaded into a fresh system and into one that had run elsewhere, each continues as the run that was not interrupted (bus cycles, picture, audio, registers, master clock) | 363 of 363 dumps, one of every board class | `famicom-state-roundtrip` |

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
