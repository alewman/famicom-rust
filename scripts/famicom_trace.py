#!/usr/bin/env python3
"""The lockstep gate's reference side: famicom-python, traced frame by frame.

Run on the reference's interpreter (external/famicom-python/.venv-pypy). Each
slice is one ``System.run_frames(1)``; its line holds the CPU registers at the
slice's end, the cycle and instruction counts, a CRC-32 of every CPU-bus cycle
of the slice as ``board.bus_observers`` sees it (packed ``t << 25 | write <<
24 | address << 8 | value``, little-endian u64), a CRC of the picture (the
9-bit values, little-endian u16), the PPU's v, t and status, and a CRC of the
slice's audio (``take_samples``, little-endian f64). ``target/release/famicom-trace``
writes the same lines; ``scripts/lockstep.py`` compares them.

    famicom_trace.py ROM --slices N [--machine KEY] [--input FILE]
                     [--reset FRAME ...] [--detail FRAME] --out FILE

``--input``: lines ``FRAME PAD BUTTONS`` (hex), applied before that slice.
``--detail FRAME``: also write every bus cycle of that slice, one per line.
"""

from __future__ import annotations

import argparse
import struct
import sys
import zlib
from array import array

from famicom_python.cartridge import read_rom_file
from famicom_python.machine import MACHINES
from famicom_python.system import System


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("rom")
    parser.add_argument("--slices", type=int, required=True)
    parser.add_argument("--machine", default="nes")
    parser.add_argument("--input")
    parser.add_argument("--reset", type=int, action="append", default=[])
    parser.add_argument("--detail", type=int)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    inputs: dict[int, list[tuple[int, int]]] = {}
    if args.input:
        with open(args.input) as handle:
            for text in handle:
                parts = text.split()
                if len(parts) == 3 and not text.startswith("#"):
                    inputs.setdefault(int(parts[0]), []).append((int(parts[1]), int(parts[2], 16)))

    system = System.from_rom(read_rom_file(args.rom), MACHINES[args.machine])
    board, cpu, ppu = system.board, system.cpu, system.board.ppu
    log = array("Q")
    append = log.append

    def observe(kind: str, address: int, value: int, t: int) -> None:
        append(t << 25 | (kind == "write") << 24 | address << 8 | value)

    board.bus_observers.append(observe)
    with open(args.out, "w") as out:
        for frame in range(args.slices):
            for pad, buttons in inputs.get(frame, ()):
                board.pads[pad].buttons = buttons
            if frame in args.reset:
                system.reset()
            del log[:]
            system.run_frames(1)
            if sys.byteorder != "little":
                log.byteswap()
            bus = zlib.crc32(log.tobytes())
            picture = array("H", ppu.picture)
            if sys.byteorder != "little":
                picture.byteswap()
            pic = zlib.crc32(picture.tobytes())
            samples = system.take_samples()
            snd = zlib.crc32(struct.pack(f"<{len(samples)}d", *samples))
            out.write(
                f"{frame} cyc {board.cycles} ins {system.instructions} "
                f"pc {cpu.pc:04X} a {cpu.a:02X} x {cpu.x:02X} y {cpu.y:02X} "
                f"s {cpu.s:02X} p {cpu.p:02X} bus {bus:08X} {len(log)} "
                f"pic {pic:08X} v {ppu.v:04X} t {ppu.t:04X} st {ppu.status:02X} "
                f"snd {snd:08X} {len(samples)}\n"
            )
            if frame == args.detail:
                for packed in log:
                    t, rest = packed >> 25, packed & 0x1FFFFFF
                    kind = "W" if rest >> 24 else "R"
                    out.write(f"  {t} {kind} {(rest >> 8) & 0xFFFF:04X} {rest & 0xFF:02X}\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
