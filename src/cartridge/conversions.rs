//! Transcription of the reference's `cartridge/conversions.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 347: `KS7030`
//! - 368: `Yung08`
//! - 415: `Board0353`
//! - 418: `LH42`
//! - 535: `LH53`
//! - 538: `SoccerChampion538`
//! - 539: `Palthena539`
//! - 548: `CTC15`
//! - 549: `KS7016B`
//! - 554: `KS7010`
//! - 557: `LH42`

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
