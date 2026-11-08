//! Transcription of the reference's `cartridge/chinese.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 14: `SL1632`
//! - 27: `MiHunChe27`
//! - 107: `MagicDragon107`
//! - 108: `FDSConversion108`
//! - 112: `Asder112`
//! - 116: `Huang116`
//! - 117: `FutureMedia117`
//! - 156: `Daou156`
//! - 177: `Hengge`
//! - 178: `Education178`
//! - 190: `Zemina190`
//! - 193: `TC112`
//! - 240: `Latch240`
//! - 241: `Hengge`
//! - 244: `Decathlon244`
//! - 246: `Fengshenbang246`
//! - 328: `RT01`
//! - 330: `Sangokushi330`
//! - 417: `FineStudio417`
//! - 515: `Noraebang515`
//! - 518: `Dance2000`
//! - 551: `Jncota551`

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
