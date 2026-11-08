//! The cartridge connector, as signals: the only thing a mapper knows of the
//! console (`cartridge/connector.py`; the protocol is described there).
//!
//! Differences forced by Rust: the reference's class attributes
//! (`snoops_cpu_reads`, `counts_m2`, ...) are methods; `irq` is a method the
//! board samples; "not driven" is `None`. A CPU read that drives only some
//! data lines returns `driven | undriven_mask << 8`, as in the reference. The
//! reference's `audio_out` (the APU, whose `expansion` the chip sets) becomes
//! [`Cartridge::expansion`], the level the chip last set, which the board
//! copies to the APU each cycle when the console routes cartridge audio.

/// A full-volume APU square, alone, in the mixer's units.
pub const APU_SQUARE: f64 = 95.88 / (8128.0 / 15.0 + 100.0);

/// Mixer units per step of a chip whose full-volume square is `full_square`
/// steps and sounds `db` dB against a full-volume APU square.
pub fn expansion_gain(db: f64, full_square: i32) -> f64 {
    APU_SQUARE * 10f64.powf(db / 20.0) / full_square as f64
}

#[allow(unused_variables)]
pub trait Cartridge {
    // -- what the cartridge asks to see ------------------------------------

    /// Also receive reads of $0000-$401F.
    fn snoops_cpu_reads(&self) -> bool {
        false
    }
    /// Receive `m2(t)` every CPU cycle.
    fn counts_m2(&self) -> bool {
        false
    }
    /// Receive `read_data()` after reads of $4020-$FFFF.
    fn latches_reads(&self) -> bool {
        false
    }
    /// Receive `ppu_address(address, t)` on every ALE.
    fn watches_ppu_address(&self) -> bool {
        false
    }
    /// Has a sound chip: the board may route its sound.
    fn expansion_audio(&self) -> bool {
        false
    }

    // -- the CPU side ------------------------------------------------------

    fn cpu_read(&mut self, address: u16, t: i64) -> Option<u16> {
        None
    }
    fn cpu_write(&mut self, address: u16, value: u8, t: i64) {}
    fn read_data(&mut self, address: u16, value: u8, t: i64) {}
    fn m2_gap(&mut self, t: i64) {}
    fn m2(&mut self, t: i64) {}

    // -- the PPU side ------------------------------------------------------

    fn ppu_address(&mut self, address: u16, t: i64) {}
    fn ciram_a10(&mut self, address: u16) -> Option<u8> {
        None
    }
    fn ppu_read(&mut self, address: u16, t: i64) -> Option<u8> {
        None
    }
    fn ppu_write(&mut self, address: u16, value: u8, t: i64) {}

    // -- outputs the board samples -----------------------------------------

    /// /IRQ, wired-OR on the board: true asserts it.
    fn irq(&self) -> bool {
        false
    }
    /// The sound chip's level, in the APU mixer's units (0 when silent).
    fn expansion(&self) -> f64 {
        0.0
    }
    /// The board connected the chip's output (the reference's `audio_out =
    /// apu`): a chip that mixes only when connected is told here.
    fn connect_audio(&mut self) {}
    fn battery_ram(&self) -> Option<Vec<u8>> {
        None
    }
    fn restore_battery(&mut self, data: &[u8]) -> bool {
        false
    }
}

/// A cartridge with nothing in it (the connector's defaults).
pub struct Empty;

impl Cartridge for Empty {}
