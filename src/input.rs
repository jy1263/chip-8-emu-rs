use glium::glutin::event::{KeyboardInput, ElementState, VirtualKeyCode};

use crate::chip8::Chip8;

static KEYMAP: [u8; 16] = [
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

pub fn parse_input(input: KeyboardInput, chip8inst: &mut Chip8) {
    let pressed = (input.state == ElementState::Pressed) as u8;
    match input.virtual_keycode.unwrap_or(VirtualKeyCode::L) {
        VirtualKeyCode::Key1=> {
            chip8inst.keystate[0x1] = pressed;
        },
        VirtualKeyCode::Key2=> {
            chip8inst.keystate[0x2] = pressed;
        },
        VirtualKeyCode::Key3=> {
            chip8inst.keystate[0x3] = pressed;
        },
        VirtualKeyCode::Key4=> {
            chip8inst.keystate[0xC] = pressed;
        },

        VirtualKeyCode::Q=> {
            chip8inst.keystate[0x4] = pressed;
        },
        VirtualKeyCode::W=> {
            chip8inst.keystate[0x5] = pressed;
        },
        VirtualKeyCode::E=> {
            chip8inst.keystate[0x6] = pressed;
        },
        VirtualKeyCode::R=> {
            chip8inst.keystate[0xD] = pressed;
        },

        VirtualKeyCode::A=> {
            chip8inst.keystate[0x7] = pressed;
        },
        VirtualKeyCode::S=> {
            chip8inst.keystate[0x8] = pressed;
        },
        VirtualKeyCode::D=> {
            chip8inst.keystate[0x9] = pressed;
        },
        VirtualKeyCode::F=> {
            chip8inst.keystate[0xE] = pressed;
        },

        VirtualKeyCode::Z=> {
            chip8inst.keystate[0xA] = pressed;
        },
        VirtualKeyCode::X=> {
            chip8inst.keystate[0x0] = pressed;
        },
        VirtualKeyCode::C=> {
            chip8inst.keystate[0xB] = pressed;
        },
        VirtualKeyCode::V=> {
            chip8inst.keystate[0xF] = pressed;
        },

        _ => {}
    }
}