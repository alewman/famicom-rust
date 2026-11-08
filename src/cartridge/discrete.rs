//! Discrete-logic boards (`discrete.py`).

use super::banked::Banked;
use super::ines::RomImage;

/// Mapper 0: 16 or 32 KiB of PRG (16 KiB mirrored), 8 KiB of CHR, no
/// registers (NROM.wiki). 8 KiB PRG images exist and mirror four times.
pub fn nrom(image: RomImage, cpu_cycle_ticks: i64) -> Banked {
    let mut b = Banked::new(image, cpu_cycle_ticks);
    nrom_map(&mut b);
    b
}

/// NROM's windows, which the boards built on it start from.
pub fn nrom_map(b: &mut Banked) {
    for window in 0..4 {
        b.map_prg(window, 8, window);
    }
    b.map_chr(0, 8, 0);
}
