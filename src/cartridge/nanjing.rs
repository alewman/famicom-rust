//! Transcription of the reference's `cartridge/nanjing.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 162: `FS304`
//! - 163: `FC001`
//! - 164: `Dongda164`
//! - 558: `YC0309`

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
