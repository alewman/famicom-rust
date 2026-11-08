//! Transcription of the reference's `cartridge/jaleco.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 18: `SS88006`
//! - 72: `JF17`
//! - 78: `JF16`
//! - 86: `JF13`
//! - 92: `JF17`
//! - 140: `JF11`

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
