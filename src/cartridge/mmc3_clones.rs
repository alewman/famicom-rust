//! Transcription of the reference's `cartridge/mmc3_clones.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 12: `SL5020B`
//! - 45: `GA23C`
//! - 52: `Outer52`
//! - 114: `SuperGame`
//! - 121: `A9711`
//! - 123: `SuperGame`
//! - 126: `ING003C`
//! - 134: `T4A54A`
//! - 165: `Latched165`
//! - 182: `SuperGame`
//! - 187: `SuperGame`
//! - 189: `PRG32Override`
//! - 196: `MRCM196`
//! - 197: `Rewired197`
//! - 205: `JC016`
//! - 208: `Gouder208`
//! - 215: `SuperGame`
//! - 219: `A9461`
//! - 238: `ContraFighter238`
//! - 245: `F003`
//! - 250: `Nitra250`
//! - 254: `XorRAM254`
//! - 258: `Board158B`
//! - 260: `HPxx260`
//! - 262: `SachenShero`
//! - 263: `Rex263`
//! - 292: `BMW8544`
//! - 313: `ResetTxROM`
//! - 334: `Multicart334`
//! - 399: `BATMAP399`
//! - 406: `Haradius406`
//! - 412: `Henggedianzi412`
//! - 422: `ING003C`
//! - 451: `Haratyler451`
//! - 512: `Zhongguo512`
//! - 513: `Board513`
//! - 516: `Cocoma516`
//! - 534: `ING003C`
//! - 594: `Rinco594`

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
