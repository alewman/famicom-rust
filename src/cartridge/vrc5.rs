//! Transcription of the reference's `cartridge/vrc5.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 547: `QTa`

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
