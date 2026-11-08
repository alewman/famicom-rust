//! Transcription of the reference's `cartridge/txc.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 36: `ProtectedLatch`
//! - 132: `ProtectedLatch`
//! - 136: `ProtectedLatch`
//! - 147: `ProtectedLatch`
//! - 172: `ProtectedLatch`
//! - 173: `ProtectedLatch`

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
