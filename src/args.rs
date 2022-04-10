use clap::{Command, Arg};

pub struct Flags {
    pub invert_colors: u8,
    pub rom_path: String,
    pub hz: u64,
    pub fg: Rgb,
    pub bg: Rgb
}

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub fn parse_args() -> Flags {
    let m = Command::new(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .about("Interpretting Emulator for Chip-8")
    .arg(Arg::new("rom_path").required(true).help("The path of the ROM that is to be loaded into the emulator."))
    .arg(Arg::new("invert_colors").required(false).short('i').long("invert-colors").help("Invert colors of the screen of the emulator."))
    .arg(Arg::new("hz").required(false).short('h').long("hz").help("The amount of loops that the emulator runs in one second.").default_value("500"))
    .arg(Arg::new("foreground_color").required(false).short('f').long("fg").help("The color in Hex that will be the foreground color.").default_value("FFFFFF"))
    .arg(Arg::new("background_color").required(false).short('b').long("bg").help("The color in Hex that will be the background color.").default_value("000000"))
    .get_matches();

    return Flags {
        invert_colors: m.is_present("invert_colors") as u8,
        rom_path: m.value_of("rom_path").unwrap().to_string(),
        hz: m.value_of("hz").unwrap().parse::<u64>().unwrap(),
        fg: hex_to_rgb(u32::from_str_radix(m.value_of("foreground_color").unwrap(), 16).unwrap()),
        bg: hex_to_rgb(u32::from_str_radix(m.value_of("background_color").unwrap(), 16).unwrap()),
    };
}

fn hex_to_rgb(hex: u32) -> Rgb {
    return Rgb {
        r: (hex >> 16) as u8,
        g: (hex >> 8) as u8,
        b: hex as u8
    };
}