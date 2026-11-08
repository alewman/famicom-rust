//! Transcription of the reference's `cartridge/multicart.py` (pending).
//!
//! The mapper numbers [`build`] answers, and the reference's class for each:
//!
//! - 15: `K1029`
//! - 57: `GK57`
//! - 58: `AddressLatch58`
//! - 59: `AddressLatch59`
//! - 60: `ResetMulticart60`
//! - 61: `Latch61`
//! - 62: `Super700`
//! - 63: `Powerful63`
//! - 81: `NTDEC81`
//! - 122: `JY043`
//! - 212: `AddressLatch212`
//! - 213: `AddressLatch58`
//! - 221: `N625092`
//! - 225: `AddressLatch225`
//! - 226: `Latch226`
//! - 227: `AddressLatch242`
//! - 229: `Latch229`
//! - 234: `Maxi15`
//! - 235: `GoldenGame235`
//! - 242: `AddressLatch242`
//! - 255: `AddressLatch225`
//! - 285: `A65AS`
//! - 286: `BS5`
//! - 289: `Champion289`
//! - 290: `Asder290`
//! - 299: `Latch299`
//! - 301: `AddressLatch242`
//! - 314: `NoRepeat314`
//! - 319: `HP898F`
//! - 332: `WS332`
//! - 359: `SB5013`
//! - 374: `ResetMMC1`
//! - 380: `Latch380`
//! - 389: `Caltron389`
//! - 396: `YY850437C`
//! - 400: `RetroUSB400`
//! - 409: `DPCMCart409`
//! - 471: `ImpactSoft471`
//! - 493: `Maxivision493`
//! - 519: `Latch519`
//! - 540: `SB5013`
//! - 541: `LittleCom541`

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
