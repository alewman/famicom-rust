//! The lockstep gate's Rust side: the same lines as scripts/famicom_trace.py
//! writes for the reference (see there for the format).
//!
//!     famicom-trace ROM --slices N [--machine KEY] [--input FILE]
//!                   [--reset FRAME ...] [--detail FRAME] --out FILE

use famicom_rust::joypad::StandardController;
use famicom_rust::{Machine, System};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

fn crc(bytes: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(bytes);
    hasher.finalize()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut rom = None;
    let mut slices = 0u64;
    let mut machine = "nes".to_string();
    let mut input = None;
    let mut resets = Vec::new();
    let mut detail = None;
    let mut out = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--slices" => slices = args.next().unwrap().parse().unwrap(),
            "--machine" => machine = args.next().unwrap(),
            "--input" => input = args.next(),
            "--reset" => resets.push(args.next().unwrap().parse::<u64>().unwrap()),
            "--detail" => detail = Some(args.next().unwrap().parse::<u64>().unwrap()),
            "--out" => out = args.next(),
            _ => rom = Some(arg),
        }
    }
    let rom = PathBuf::from(rom.expect("ROM"));
    let mut inputs: HashMap<u64, Vec<(usize, u8)>> = HashMap::new();
    if let Some(path) = input {
        for text in std::fs::read_to_string(path).unwrap().lines() {
            let parts: Vec<&str> = text.split_whitespace().collect();
            if parts.len() == 3 && !text.starts_with('#') {
                inputs.entry(parts[0].parse().unwrap()).or_default().push((
                    parts[1].parse().unwrap(),
                    u8::from_str_radix(parts[2], 16).unwrap(),
                ));
            }
        }
    }
    let machine = Machine::by_key(&machine).expect("machine");
    let mut system = match System::from_path(&rom, machine) {
        Ok(system) => system,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    system.board().bus_log = Some(Vec::with_capacity(40_000));
    let mut out = std::io::BufWriter::new(std::fs::File::create(out.expect("--out")).unwrap());
    let mut bytes = Vec::new();
    for frame in 0..slices {
        if let Some(list) = inputs.get(&frame) {
            for &(pad, buttons) in list {
                let device = system.board().pads[pad].as_any_mut();
                if let Some(pad) = device.downcast_mut::<StandardController>() {
                    pad.buttons = buttons;
                }
            }
        }
        if resets.contains(&frame) {
            system.reset();
        }
        system.board().bus_log.as_mut().unwrap().clear();
        system.run_frames(1);
        let samples = system.take_samples();
        let board = &system.cpu.bus;
        let log = board.bus_log.as_ref().unwrap();
        bytes.clear();
        for packed in log {
            bytes.extend_from_slice(&packed.to_le_bytes());
        }
        let bus = crc(&bytes);
        bytes.clear();
        for value in &board.ppu.picture {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        let pic = crc(&bytes);
        bytes.clear();
        for sample in &samples {
            bytes.extend_from_slice(&sample.to_le_bytes());
        }
        let snd = crc(&bytes);
        let cpu = &system.cpu;
        let ppu = &board.ppu;
        writeln!(
            out,
            "{frame} cyc {} ins {} pc {:04X} a {:02X} x {:02X} y {:02X} s {:02X} p {:02X} \
             bus {bus:08X} {} pic {pic:08X} v {:04X} t {:04X} st {:02X} snd {snd:08X} {}",
            board.cycles(),
            system.instructions,
            cpu.pc,
            cpu.a,
            cpu.x,
            cpu.y,
            cpu.s,
            cpu.p,
            log.len(),
            ppu.v,
            ppu.t,
            ppu.status,
            samples.len()
        )
        .unwrap();
        if detail == Some(frame) {
            for &packed in log {
                let t = packed >> 25;
                let rest = packed & 0x1FF_FFFF;
                let kind = if rest >> 24 != 0 { "W" } else { "R" };
                writeln!(
                    out,
                    "  {t} {kind} {:04X} {:02X}",
                    (rest >> 8) & 0xFFFF,
                    rest & 0xFF
                )
                .unwrap();
            }
        }
    }
}
