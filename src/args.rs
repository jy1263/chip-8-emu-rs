use crate::chip8::Chip8;
use clap::{Parser, App, Command, Arg};

pub struct Flags {
    pub invert_colors: u8,
    pub rom_path: String
}

pub fn parse_args() -> Flags {
    let m = Command::new("Emulator")
    .author("Amy Y")
    .version("0.1.0")
    .about("Interpretting Emulator for Chip-8")
    .arg(Arg::new("rom_path").required(true).help("The path of the ROM that is to be loaded into the emulator."))
    .arg(Arg::new("invert_colors").required(false).short('i').long("invert-colors").help("Invert colors of the screen of the emulator."))
    .get_matches();

    return Flags {
        invert_colors: m.is_present("invert_colors") as u8,
        rom_path: m.value_of("rom_path").unwrap().to_string()
    };
}