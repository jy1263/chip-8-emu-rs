use crate::{chip8::Chip8, args::Flags, fstools::{self, load_state}};

static KEYMAP: [usize; 16] = [
    0x1, // 1
    0x2, // 2
    0x3, // 3
    0xC, // C
    0x4, // 4
    0x5, // 5
    0x6, // 6
    0xD, // D
    0x7, // 7
    0x8, // 8
    0x9, // 9
    0xE, // E
    0xA, // A
    0x0, // 0
    0xB, // B
    0xF, // F
];

// pub fn parse_input(input: KeyboardInput, chip8inst: &mut Chip8, flags: &Flags) {
//     let pressed = (input.state == ElementState::Pressed) as u8;
//     match input.virtual_keycode.unwrap_or(VirtualKeyCode::L) {
//         VirtualKeyCode::Key1=> {
//             chip8inst.keystate[KEYMAP[0]] = pressed;
//         },
//         VirtualKeyCode::Key2=> {
//             chip8inst.keystate[KEYMAP[1]] = pressed;
//         },
//         VirtualKeyCode::Key3=> {
//             chip8inst.keystate[KEYMAP[2]] = pressed;
//         },
//         VirtualKeyCode::Key4=> {
//             chip8inst.keystate[KEYMAP[3]] = pressed;
//         },

//         VirtualKeyCode::Q=> {
//             chip8inst.keystate[KEYMAP[4]] = pressed;
//         },
//         VirtualKeyCode::W=> {
//             chip8inst.keystate[KEYMAP[5]] = pressed;
//         },
//         VirtualKeyCode::E=> {
//             chip8inst.keystate[KEYMAP[6]] = pressed;
//         },
//         VirtualKeyCode::R=> {
//             chip8inst.keystate[KEYMAP[7]] = pressed;
//         },

//         VirtualKeyCode::A=> {
//             chip8inst.keystate[KEYMAP[8]] = pressed;
//         },
//         VirtualKeyCode::S=> {
//             chip8inst.keystate[KEYMAP[9]] = pressed;
//         },
//         VirtualKeyCode::D=> {
//             chip8inst.keystate[KEYMAP[10]] = pressed;
//         },
//         VirtualKeyCode::F=> {
//             chip8inst.keystate[KEYMAP[11]] = pressed;
//         },

//         VirtualKeyCode::Z=> {
//             chip8inst.keystate[KEYMAP[12]] = pressed;
//         },
//         VirtualKeyCode::X=> {
//             chip8inst.keystate[KEYMAP[13]] = pressed;
//         },
//         VirtualKeyCode::C=> {
//             chip8inst.keystate[KEYMAP[14]] = pressed;
//         },
//         VirtualKeyCode::V=> {
//             chip8inst.keystate[KEYMAP[15]] = pressed;
//         },



//         // save state
//         VirtualKeyCode::F5 => {
//             if pressed == 1 {
//                 let rompath = std::path::Path::new(flags.rom_path.as_str());
//                 let statepath = rompath.with_extension("state");

//                 fstools::save_state(&statepath, chip8inst);
//             }
//         },
//         VirtualKeyCode::F6 => {
//             if pressed == 1 {
//                 let rompath = std::path::Path::new(flags.rom_path.as_str());
//                 let statepath = rompath.with_extension("state");

//                 load_state(&statepath, chip8inst)
//             }
//         },
//         _ => {}
//     }
// }