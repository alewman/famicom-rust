//! Transcription of the reference's `cartridge/waixing.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 74: `MixedCHR`
//! - 176: `FK23C`
//! - 191: `MixedCHR`
//! - 192: `MixedCHR`
//! - 194: `MixedCHR`
//! - 195: `MixedCHR`
//! - 198: `Xiangfeng198`
//! - 199: `FS309`
//! - 249: `T9552`
//! - 252: `WaixingVRC4`
//! - 253: `WaixingVRC4`
//! - 523: `Jncota523`

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
