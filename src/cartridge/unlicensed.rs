//! Transcription of the reference's `cartridge/unlicensed.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 11: `ColorDreams`
//! - 38: `BitCorp38`
//! - 41: `Caltron41`
//! - 46: `RumbleStation46`
//! - 71: `Camerica`
//! - 79: `NINA03`
//! - 104: `Pegasus104`
//! - 113: `NINA03`
//! - 144: `DeathRace`
//! - 167: `Subor167`
//! - 168: `Racermate`
//! - 216: `RCM216`
//! - 228: `Action52`
//! - 232: `Camerica`
//! - 487: `NINA08`

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
