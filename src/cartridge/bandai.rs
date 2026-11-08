//! Transcription of the reference's `cartridge/bandai.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 16: `BandaiFCG`
//! - 70: `Bandai74161`
//! - 96: `OekaKids`
//! - 152: `Bandai74161`
//! - 153: `BandaiFCGRAM`
//! - 157: `Datach`
//! - 159: `BandaiX24C01`
//! - 188: `Karaoke`

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
