//! Transcription of the reference's `cartridge/kaiser.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 40: `NTDEC2722`
//! - 42: `Kaiser42`
//! - 43: `TonyI43`
//! - 50: `N32Mapper50`
//! - 56: `KaiserSMB3`
//! - 103: `DokiDoki103`
//! - 106: `SMB3Bootleg106`
//! - 120: `LH15`
//! - 125: `Whirlwind125`
//! - 142: `KS7032`
//! - 302: `KS7057`
//! - 303: `Kaiser303`
//! - 304: `Board09034A`
//! - 305: `Kaiser305`
//! - 306: `Kaiser306`
//! - 307: `Kaiser307`
//! - 309: `Whirlwind309`
//! - 312: `KS7013B`
//! - 326: `Contra326`
//! - 346: `KS7012`
//! - 521: `Board521`
//! - 522: `Kaiser522`
//! - 525: `KS7021A`
//! - 526: `Board526`

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
