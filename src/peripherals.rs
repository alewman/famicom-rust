//! The devices on the controller ports and the expansion port, as the board
//! sees them (`peripherals.py`): two protocols.
//!
//! - a **port device** (`board.pads[0]` or `[1]`), like the standard
//!   controller: `set_strobe` with OUT0, `read` D0-D4 at its /OE, `clock` at
//!   the end of the /OE pulse;
//! - an **expansion device** (`board.expansion`): `set_out` with OUT0-2,
//!   `read(port)` and `clock(port)` for either /OE.
//!
//! Difference forced by Rust: the reference's devices that look at the
//! picture keep a reference to the PPU; here every call is handed the PPU.

use crate::joypad::StandardController;
use crate::ppu::Ppu;
use std::any::Any;

pub trait PortDevice {
    fn set_strobe(&mut self, high: bool, ppu: &Ppu);
    fn read(&mut self, ppu: &Ppu) -> u8;
    fn clock(&mut self, ppu: &Ppu);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ExpansionDevice {
    fn set_out(&mut self, value: u8, ppu: &Ppu);
    fn read(&mut self, port: usize, ppu: &Ppu) -> u8;
    fn clock(&mut self, port: usize, ppu: &Ppu);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl PortDevice for StandardController {
    fn set_strobe(&mut self, high: bool, _: &Ppu) {
        StandardController::set_strobe(self, high);
    }
    fn read(&mut self, _: &Ppu) -> u8 {
        StandardController::read(self)
    }
    fn clock(&mut self, _: &Ppu) {
        StandardController::clock(self);
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// NES 2.0 expansion-device numbers (`peripherals.py`).
pub const STANDARD: u8 = 0x01;
pub const FOUR_SCORE: u8 = 0x02;
pub const TWO_ZAPPERS: u8 = 0x09;
pub const POWER_PAD_A: u8 = 0x0B;
pub const POWER_PAD_B: u8 = 0x0C;
pub const VAUS_NES: u8 = 0x0F;

/// Devices only the NES's ports take (`peripherals.NES_ONLY`).
pub fn nes_only(device: u8) -> bool {
    matches!(
        device,
        FOUR_SCORE | POWER_PAD_A | POWER_PAD_B | VAUS_NES | TWO_ZAPPERS
    )
}
