//! Save states (the reference's `state.py`, at famicom-python `c497645`): the
//! whole console between two instructions, as bytes, and back.
//!
//! As in the reference, a state is what changes as the console runs, not the
//! wiring: it is loaded into a [`System`] built from the same cartridge on the
//! same machine, which keeps its configuration (`Machine`, `Timing`, the APU's
//! tables), the ROM image and the connections between the parts. Equivalence
//! is the reference's test: a run saved at an instruction boundary and loaded
//! into another system continues bus cycle for bus cycle as the run that was
//! never interrupted (`famicom-state-roundtrip`, scripts/state_roundtrip.sh).
//!
//! Differences forced by Rust: the format is the crate's own (zlib of the
//! bincode of the serde-derived structs, behind a small header), not the
//! reference's tagged JSON; a state is portable between builds of this crate, not to the
//! reference. The CPU's part is 6502-rust's `CpuState`, mirrored field for
//! field. Configuration the structs hold by value or by `&'static` reference
//! (the ROM image in `Banked`, the PPU's `Timing`, the APU's tables, the
//! cartridge behind the PPU, the devices in the ports) is not written: while
//! a state loads, [`LoadContext`] hands the decoder the system's own.

use crate::apu::{tables, Tables};
use crate::cartridge::ines::{BoardInfo, RomImage};
use crate::machine::{Timing, NTSC};
use crate::system::System;
use serde::{Deserialize, Serialize};
use sixfiveohtwo::CpuState;
use std::cell::RefCell;

pub const MAGIC: &[u8; 8] = b"FCRSTATE";
pub const VERSION: u32 = 1;

/// What a loading state takes from the system it is loaded into.
#[derive(Clone)]
pub struct LoadContext {
    pub image: RomImage,
    pub timing: Timing,
    pub tables: Tables,
}

thread_local! {
    static CONTEXT: RefCell<Option<LoadContext>> = const { RefCell::new(None) };
}

fn context<R>(f: impl FnOnce(&LoadContext) -> R) -> R {
    CONTEXT.with(|c| {
        let context = c.borrow();
        f(context
            .as_ref()
            .expect("a state decoded outside load_state"))
    })
}

/// The ROM image of the system being loaded (serde default for `Banked::image`).
pub fn live_image() -> RomImage {
    CONTEXT.with(|c| match c.borrow().as_ref() {
        Some(context) => context.image.clone(),
        None => RomImage {
            header: Vec::new(),
            trainer: Vec::new(),
            prg: Vec::new(),
            chr: Vec::new(),
            misc: Vec::new(),
            board: BoardInfo::new(0),
            nes2: false,
        },
    })
}

/// The timing of the system being loaded (serde default for `Ppu::timing`).
pub fn live_timing() -> Timing {
    CONTEXT.with(|c| {
        c.borrow()
            .as_ref()
            .map_or(NTSC, |context| context.timing.clone())
    })
}

pub fn live_tables() -> Tables {
    CONTEXT.with(|c| {
        c.borrow()
            .as_ref()
            .map_or(tables(crate::machine::ApuTables::Ntsc), |context| {
                context.tables
            })
    })
}

pub fn live_noise_periods() -> &'static [u16; 16] {
    context(|c| c.tables.noise_periods)
}

pub fn live_dmc_rates() -> &'static [u16; 16] {
    context(|c| c.tables.dmc_rates)
}

/// `Banked::prg` and `Banked::chr`: written only where they differ from what
/// the ROM image gives (flash written, CHR-RAM, a board's own layout), as the
/// reference writes the image's bytes by name, not content.
pub mod rom_bytes {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    fn from_image(chr: bool) -> Vec<u8> {
        let image = super::live_image();
        if chr {
            return image.chr;
        }
        let mut prg = image.prg;
        if !prg.is_empty() && prg.len() < 0x2000 {
            prg = prg.repeat(0x2000 / prg.len());
        }
        prg
    }

    fn save<S: Serializer>(bytes: &[u8], chr: bool, serializer: S) -> Result<S::Ok, S::Error> {
        let same = super::has_context() && !bytes.is_empty() && bytes == from_image(chr).as_slice();
        let value: Option<&[u8]> = if same { None } else { Some(bytes) };
        value.serialize(serializer)
    }

    fn load<'de, D: Deserializer<'de>>(chr: bool, deserializer: D) -> Result<Vec<u8>, D::Error> {
        let value: Option<Vec<u8>> = Option::deserialize(deserializer)?;
        Ok(value.unwrap_or_else(|| from_image(chr)))
    }

    pub mod prg {
        pub fn serialize<S: serde::Serializer>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error> {
            super::save(bytes, false, s)
        }
        pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
            super::load(false, d)
        }
    }

    pub mod chr {
        pub fn serialize<S: serde::Serializer>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error> {
            super::save(bytes, true, s)
        }
        pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
            super::load(true, d)
        }
    }
}

fn has_context() -> bool {
    CONTEXT.with(|c| c.borrow().is_some())
}

/// A placeholder while a state loads (serde default for `Ppu::cart`).
pub fn empty_cart() -> Box<dyn crate::cartridge::Cartridge> {
    Box::new(crate::cartridge::connector::Empty)
}

/// A boxed object's state, behind the trait objects the board holds (the
/// cartridge, the devices): bincode of the concrete type.
pub trait Snapshot {
    fn save_state(&self) -> Vec<u8>;
    /// Replace `self` with the state `data` holds, `self`'s type's.
    fn load_state(&mut self, data: &[u8]) -> Result<(), String>;
}

impl<T: Serialize + for<'de> Deserialize<'de>> Snapshot for T {
    fn save_state(&self) -> Vec<u8> {
        bincode::serialize(self).expect("a state serializes")
    }

    fn load_state(&mut self, data: &[u8]) -> Result<(), String> {
        *self = bincode::deserialize(data).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// 6502-rust's `CpuState`, field for field.
#[derive(Serialize, Deserialize)]
struct CpuFields {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    pc: u16,
    p: u8,
    halted: bool,
    reset_pending: bool,
    maskable_interrupt_pending: bool,
    non_maskable_interrupt_pending: bool,
    polled_i: Option<bool>,
    polled_maskable_interrupt: Option<bool>,
    polled_non_maskable_interrupt: Option<bool>,
    skipped_poll: bool,
}

impl From<CpuState> for CpuFields {
    fn from(s: CpuState) -> Self {
        CpuFields {
            a: s.a,
            x: s.x,
            y: s.y,
            s: s.s,
            pc: s.pc,
            p: s.p,
            halted: s.halted,
            reset_pending: s.reset_pending,
            maskable_interrupt_pending: s.maskable_interrupt_pending,
            non_maskable_interrupt_pending: s.non_maskable_interrupt_pending,
            polled_i: s.polled_i,
            polled_maskable_interrupt: s.polled_maskable_interrupt,
            polled_non_maskable_interrupt: s.polled_non_maskable_interrupt,
            skipped_poll: s.skipped_poll,
        }
    }
}

impl From<CpuFields> for CpuState {
    fn from(f: CpuFields) -> Self {
        CpuState {
            a: f.a,
            x: f.x,
            y: f.y,
            s: f.s,
            pc: f.pc,
            p: f.p,
            halted: f.halted,
            reset_pending: f.reset_pending,
            maskable_interrupt_pending: f.maskable_interrupt_pending,
            non_maskable_interrupt_pending: f.non_maskable_interrupt_pending,
            polled_i: f.polled_i,
            polled_maskable_interrupt: f.polled_maskable_interrupt,
            polled_non_maskable_interrupt: f.polled_non_maskable_interrupt,
            skipped_poll: f.skipped_poll,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Header {
    version: u32,
    rom_sha1: String,
    machine: String,
}

#[derive(Serialize, Deserialize)]
struct Body {
    cpu: CpuFields,
    instructions: u64,
    board: Vec<u8>,
}

fn rom_sha1(system: &System) -> String {
    match &system.image {
        Some(image) => {
            let whole = [
                image.prg.as_slice(),
                image.chr.as_slice(),
                image.misc.as_slice(),
            ]
            .concat();
            crate::cartridge::database::sha1_hex(&whole)
        }
        None => String::new(),
    }
}

/// The state of `system` as bytes (between two of its steps).
pub fn save_state(system: &System) -> Vec<u8> {
    let header = Header {
        version: VERSION,
        rom_sha1: rom_sha1(system),
        machine: system.cpu.bus.machine.name.to_string(),
    };
    let machine = &system.cpu.bus.machine;
    let context = LoadContext {
        image: system
            .cpu
            .bus
            .ppu
            .cart
            .banked()
            .map_or_else(live_image, |b| b.image.clone()),
        timing: machine.timing.clone(),
        tables: tables(machine.apu_tables),
    };
    CONTEXT.with(|c| *c.borrow_mut() = Some(context));
    let board = system.cpu.bus.save_state();
    CONTEXT.with(|c| *c.borrow_mut() = None);
    let body = Body {
        cpu: system.cpu.capture_state().into(),
        instructions: system.instructions,
        board,
    };
    let mut out = MAGIC.to_vec();
    out.extend(bincode::serialize(&header).expect("a header serializes"));
    let mut encoder = flate2::write::ZlibEncoder::new(out, flate2::Compression::new(6));
    std::io::Write::write_all(
        &mut encoder,
        &bincode::serialize(&body).expect("a state serializes"),
    )
    .expect("compression in memory");
    encoder.finish().expect("compression in memory")
}

/// A size no state of `system` exceeds: what a frontend that wants a fixed
/// buffer (libretro's `retro_serialize_size`) allocates. The uncompressed
/// state, with PRG and CHR written whole, and room to spare; zlib never grows
/// data by more than a fraction of a percent.
pub fn size_bound(system: &System) -> usize {
    let raw = bincode::serialized_size(&system.cpu.bus.save_state()).unwrap_or(0) as usize;
    let rom = system
        .image
        .as_ref()
        .map_or(0, |image| image.prg.len().max(0x2000) + image.chr.len());
    raw + 2 * rom + 0x10000
}

/// Put `system` in the state `data` was saved from (same cartridge and machine).
pub fn load_state(system: &mut System, data: &[u8]) -> Result<(), String> {
    if data.len() < MAGIC.len() || &data[..MAGIC.len()] != MAGIC {
        return Err("not a save state".into());
    }
    let mut rest = &data[MAGIC.len()..];
    let header: Header = bincode::deserialize_from(&mut rest).map_err(|e| e.to_string())?;
    if header.version != VERSION {
        return Err(format!(
            "a state of version {}, not {VERSION}",
            header.version
        ));
    }
    if header.rom_sha1 != rom_sha1(system) {
        return Err("a state of another cartridge".into());
    }
    let machine = &system.cpu.bus.machine;
    if header.machine != machine.name {
        return Err(format!(
            "a state of the {}, not the {}",
            header.machine, machine.name
        ));
    }
    let body: Body = bincode::deserialize_from(flate2::read::ZlibDecoder::new(rest))
        .map_err(|e| e.to_string())?;
    let context = LoadContext {
        image: system
            .cpu
            .bus
            .ppu
            .cart
            .banked()
            .map_or_else(live_image, |b| b.image.clone()),
        timing: machine.timing.clone(),
        tables: tables(machine.apu_tables),
    };
    CONTEXT.with(|c| *c.borrow_mut() = Some(context));
    let loaded = system.cpu.bus.load_state(&body.board);
    CONTEXT.with(|c| *c.borrow_mut() = None);
    loaded?;
    system.cpu.restore_state(&body.cpu.into())?;
    system.instructions = body.instructions;
    Ok(())
}
