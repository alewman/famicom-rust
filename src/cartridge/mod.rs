//! Cartridges: the connector protocol, the file loader and the mappers
//! (`cartridge/__init__.py`).
//!
//! [`load`] identifies the board in the reference's override order: a
//! caller's override, then nes20db by the whole ROM, then by PRG and CHR, then
//! the file's NES 2.0 header, then the iNES 1.0 header.

pub mod banked;
pub mod connector;
pub mod database;
pub mod discrete;
pub mod ines;

pub use banked::Banked;
pub use connector::Cartridge;
pub use ines::{BoardInfo, RomImage};

use std::io::Read;
use std::path::Path;

/// A cartridge ready to plug in, and how its board was identified.
pub struct Loaded {
    pub cartridge: Box<dyn Cartridge>,
    pub image: RomImage,
    /// "override", "nes20db", "NES 2.0 header" or "iNES header".
    pub source: &'static str,
    pub disagreements: Vec<String>,
}

/// A `.nes`, `.fds` or `.qd` file's bytes, or the one such file in a `.zip`.
pub fn read_rom_file(path: &Path, suffixes: &[&str]) -> Result<Vec<u8>, String> {
    let is_zip = path
        .extension()
        .is_some_and(|e| e.to_string_lossy().eq_ignore_ascii_case("zip"));
    if is_zip {
        let file = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| format!("{}: {e}", path.display()))?;
        let names: Vec<String> = archive
            .file_names()
            .filter(|n| {
                let lower = n.to_lowercase();
                suffixes.iter().any(|s| lower.ends_with(s))
            })
            .map(String::from)
            .collect();
        if names.len() != 1 {
            return Err(format!(
                "{}: expected one of {suffixes:?} inside, found {}",
                path.display(),
                names.len()
            ));
        }
        let mut data = Vec::new();
        archive
            .by_name(&names[0])
            .map_err(|e| e.to_string())?
            .read_to_end(&mut data)
            .map_err(|e| e.to_string())?;
        return Ok(data);
    }
    std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

pub const ROM_SUFFIXES: &[&str] = &[".nes", ".fds", ".qd"];

fn differences(winner: &BoardInfo, other: &BoardInfo, label: &str) -> Vec<String> {
    let mut out = Vec::new();
    macro_rules! field {
        ($name:ident) => {
            if winner.$name != other.$name {
                out.push(format!(
                    "{}: {label} says {:?}, using {:?}",
                    stringify!($name),
                    other.$name,
                    winner.$name
                ));
            }
        };
    }
    field!(mapper);
    field!(submapper);
    field!(mirroring);
    field!(battery);
    field!(prg_ram);
    field!(prg_nvram);
    field!(chr_ram);
    field!(chr_nvram);
    field!(timing);
    field!(console);
    if winner.expansion != other.expansion && other.expansion != 0 {
        out.push(format!(
            "expansion: {label} says {:?}, using {:?}",
            other.expansion, winner.expansion
        ));
    }
    out
}

/// Each implemented iNES mapper's board, built from the image and the CPU
/// cycle length in master ticks.
pub fn build(image: RomImage, cpu_cycle_ticks: i64) -> Result<Box<dyn Cartridge>, String> {
    let mapper = image.board.mapper;
    Ok(match mapper {
        0 => Box::new(discrete::nrom(image, cpu_cycle_ticks)),
        _ => return Err(format!("mapper {mapper} is not implemented yet")),
    })
}

/// Identify the cartridge in `data` and build its mapper.
pub fn load(
    data: &[u8],
    cpu_cycle_ticks: i64,
    override_board: Option<BoardInfo>,
    use_database: bool,
) -> Result<Loaded, String> {
    let image = ines::parse(data)?;
    let header = image.board.clone();
    let header_label = if image.nes2 {
        "NES 2.0 header"
    } else {
        "iNES header"
    };
    let database = if use_database {
        database::lookup_nes20db(&image, database::load_nes20db())
    } else {
        None
    };
    let mut disagreements = Vec::new();
    let (board, source) = if let Some(board) = override_board {
        (board, "override")
    } else if let Some(board) = database.clone() {
        (board, "nes20db")
    } else {
        (header.clone(), header_label)
    };
    // The reference compares by identity ("database is not board"): the
    // database's entry loses to an override, and the header to either.
    if source == "override" {
        if let Some(db) = &database {
            disagreements.extend(differences(&board, db, "nes20db"));
        }
    }
    if source != header_label {
        disagreements.extend(differences(&board, &header, header_label));
    }
    let image = RomImage { board, ..image };
    let cartridge = build(image.clone(), cpu_cycle_ticks)?;
    Ok(Loaded {
        cartridge,
        image,
        source,
        disagreements,
    })
}
