use std::{fs, fs::File};
use std::io::Read;

mod chip8;
mod opcodes;

use crate::chip8::Chip8;

fn main() {
    let mut chip8inst = Chip8::new();
    chip8inst.load_program(&get_file_as_byte_vec("./roms/ibm.ch8"));
    while true {
        chip8inst.single_cycle();
    }
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
