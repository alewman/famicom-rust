//! Transcription of the reference's `cartridge/homebrew.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 28: `Action53`
//! - 29: `SealieRET`
//! - 31: `NSF31`
//! - 111: `GTROM`
//! - 218: `MagicFloor`
//! - 284: `DripGame`
//! - 413: `BATMAPSRR`

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
