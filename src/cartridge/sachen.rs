//! Transcription of the reference's `cartridge/sachen.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 133: `SachenLatch`
//! - 137: `Sachen8259D`
//! - 138: `Sachen8259`
//! - 139: `Sachen8259`
//! - 141: `Sachen8259`
//! - 143: `Sachen143`
//! - 145: `SachenLatch`
//! - 148: `SachenLatch`
//! - 149: `SachenLatch`
//! - 150: `SachenLS374N`
//! - 243: `SachenLS374N`
//! - 533: `Sachen3014`
//! - 553: `Sachen3013`

use super::connector::Cartridge;
use super::ines::RomImage;

/// The board for `mapper` if this module builds it.
pub fn build(
    mapper: u16,
    image: RomImage,
    cpu_cycle_ticks: i64,
) -> Result<Box<dyn Cartridge>, RomImage> {
    let _ = (mapper, cpu_cycle_ticks);
    Err(image)
}
