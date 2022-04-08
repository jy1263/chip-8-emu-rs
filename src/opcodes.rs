use crate::chip8::Chip8;

// pub fn parse_op(chip8: &mut Chip8) {
//     if chip8.opcode != 0 {
//         match chip8.opcode  {
//             0x00E0 => println!("CLS"),
//             0x00EE => {
//                 // return from subroutine
//                 println!("RET")
//             },
//             0 => {},
//             _ => {}
//         }
        
//         // matches opcode with last 3 hex digits removed (ie, 0xA22A -> 0xA000)
//         match chip8.opcode & 0xF000 {
//             0x1000 => {
//                 // 1NNN - jump to address NNN
//                 println!("1NNN")
//             }
//             0x2000 => {
//                 // 2NNN - call subroutine at NNN
//                 println!("2NNN")
//             }
//             0x3000 => {
//                 // 3XNN - skip next instruction if VX == NN
//                 println!("3XNN")
//             }
//             0x4000 => {
//                 // 4XNN - skip next instruction if VX != NN
//                 println!("4XNN")
//             }
//             0x6000 => {
//                 // 6XNN - set VX to NN
//                 println!("6XNN")
//             }
//             0x7000 => {
//                 // 7XNN - add NN to VX
//                 println!("7XNN")
//             }
//             _ => {}
//         }
//     }
// }

pub fn parse_op(chip8: &mut Chip8) {
    if chip8.opcode != 0 {
        let x = (chip8.opcode & 0x0F00) >> 8;
        let y = (chip8.opcode & 0x00F0) >> 4;

        match chip8.opcode  {
            0x00E0 => println!("CLS"),
            0x00EE => {
                // return from subroutine
                println!("RET")
            },
            0 => {},
            _ => {}
        }

        // matches opcode with last 3 nibbles removed (ie, 0xA22A -> 0xA000)
        match chip8.opcode & 0xF000 {
            0x1000 => {
                // 1NNN - jump to address NNN
                println!("1NNN")
            }
            0x2000 => {
                // 2NNN - call subroutine at NNN
                println!("2NNN")
            }
            0x3000 => {
                // 3XNN - skip next instruction if VX == NN
                println!("3XNN")
            }
            0x4000 => {
                // 4XNN - skip next instruction if VX != NN
                println!("4XNN")
            }
            0x6000 => {
                // 6XNN - set VX to NN
                println!("6XNN")
            }
            0x7000 => {
                // 7XNN - add NN to VX
                println!("7XNN")
            },
            0xA000 => {

            },
            0xB000 => {

            },
            0xC000 => {

            },
            0xD000 => {

            },
            _ => {}
        }

        // matches first and last nibbles (ie, 0xA22A -> 0xA00A)
        match chip8.opcode & 0xF00F {
            0x5000 => {
                // 5XY0 - skip next instruction if VX == VY
                println!("5XY0")
            },
            0x8000 => {
                // 8XY0 - set VX to VY
                println!("8XY0")
            },
            0x8001 => {
                // 8XY1 - set VX to VX | VY
                println!("8XY1")
            },
            0x8002 => {
                // 8XY2 - set VX to VX & VY
                println!("8XY2")
            },
            0x8003 => {
                // 8XY3 - set VX to VX ^ VY
                println!("8XY3")
            },
            0x8004 => {
                // 8XY4 - set VX to VX + VY, set VF to 1 if carry
                println!("8XY4")
            },
            0x8005 => {
                // 8XY5 - set VX to VX - VY, set VF to 0 if borrow
                println!("8XY5")
            },
            0x8006 => {
                // 8XY6 - set VX to VX >> 1, set VF to LSB of VX
                println!("8XY6")
            },
            0x8007 => {
                // 8XY7 - set VX to VY - VX, set VF to 0 if borrow
                println!("8XY7")
            },
            0x800E => {
                // 8XYE - set VX to VX << 1, set VF to MSB of VX
                println!("8XYE")
            },
            0x9000 => {
                // 9XY0 - skip next instruction if VX != VY
                println!("9XY0")
            },
            _ => {}
        }

        match chip8.opcode & 0xF0FF {
            0xE09E => {
                // EX9E - skip next instruction if key in VX is pressed
                println!("EX9E")
            },
            0xE0A1 => {
                // EXA1 - skip next instruction if key in VX is not pressed
                println!("EXA1")
            },
            0xF007 => {
                // FX07 - set VX to delay timer value
                println!("FX07")
            },
            0xF00A => {
                // FX0A - wait for keypress, store in VX
                println!("FX0A")
            },
            0xF015 => {
                // FX15 - set delay timer to VX
                println!("FX15")
            },
            0xF018 => {
                // FX18 - set sound timer to VX
                println!("FX18")
            },
            0xF01E => {
                // FX1E - add VX to I
                println!("FX1E")
            },
            0xF029 => {
                // FX29 - set I to location of sprite for digit VX
                println!("FX29")
            },
            0xF033 => {
                // FX33 - store BCD representation of VX in memory locations I, I+1, and I+2
                println!("FX33")
            },
            0xF055 => {
                // FX55 - store V0 to VX in memory starting at address I
                println!("FX55")
            },
            0xF065 => {
                // FX65 - read V0 to VX from memory starting at address I
                println!("FX65")
            },
            _ => {}
        
        }
    }
}