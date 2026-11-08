//! Transcription of the reference's `cartridge/konami.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 21: `VRC24`
//! - 22: `VRC24`
//! - 23: `VRC24`
//! - 24: `VRC6`
//! - 25: `VRC24`
//! - 26: `VRC6`
//! - 73: `VRC3`
//! - 75: `VRC1`
//! - 85: `VRC7`
//! - 183: `ShuiGuan183`
//! - 222: `Ninja222`
//! - 266: `CityFighter266`
//! - 272: `KidDracula272`
//! - 298: `TF1201`
//! - 308: `TH2131`
//! - 529: `T230`
//! - 530: `AX5705`
//! - 544: `FS306`

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
