//! Transcription of the reference's `cartridge/namco.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 19: `Namco163`
//! - 76: `Namco3446`
//! - 88: `Namco3433`
//! - 95: `Namco3425`
//! - 154: `Namco3433`
//! - 206: `Namco108`
//! - 210: `Namco175`

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
