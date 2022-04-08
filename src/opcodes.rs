use crate::chip8::Chip8;
use rand::{Rng, prelude::ThreadRng};

pub fn parse_op(chip8: &mut Chip8) {
    if chip8.opcode != 0 {
        let x = (chip8.opcode & 0x0F00) >> 8;
        let y = (chip8.opcode & 0x00F0) >> 4;
        let nn = (chip8.opcode & 0x00FF) as u8;

        match chip8.opcode  {
            0x00E0 => {
                chip8.display = [0; 2048];
                return;
            },
            0x00EE => {
                // sets pc to the address at the top of the stack
                chip8.pc = chip8.jumpstack[chip8.stackpointer as usize];
                chip8.stackpointer -= 1;
                println!("RET");
                return;
            },
            0 => {},
            _ => {}
        }

        // matches opcode with last 3 nibbles removed (ie, 0xA22A -> 0xA000)
        match chip8.opcode & 0xF000 {
            0x0000 => {
                // ignored by modern interpreters
                return;
            },
            0x1000 => {
                // 1NNN - jump to address NNN
                chip8.pc = chip8.opcode & 0x0FFF;
                return;
            }
            0x2000 => {
                // 2NNN - call subroutine at NNN
                chip8.stackpointer += 1;
                chip8.jumpstack[chip8.stackpointer as usize] = chip8.pc;
                chip8.pc = chip8.opcode & 0x0FFF;
                return;
            }
            0x3000 => {
                // 3XNN - skip next instruction if VX == NN
                if chip8.vregisters[x as usize] == nn {
                    chip8.pc += 2;
                }
            }
            0x4000 => {
                // 4XNN - skip next instruction if VX != NN
                if chip8.vregisters[x as usize] != nn {
                    chip8.pc += 2;
                }
                return;
            }
            0x6000 => {
                // 6XNN - set VX to NN
                chip8.vregisters[x as usize] = nn;
                return;
            }
            0x7000 => {
                // 7XNN - add NN to VX
                chip8.vregisters[x as usize] += nn;
                return;
            },
            0xA000 => {
                // ANNN - set I to NNN
                chip8.i = chip8.opcode & 0x0FFF;
                return;
            },
            0xB000 => {
                // BNNN - jump to address NNN + V0
                chip8.pc = (chip8.opcode & 0x0FFF) + chip8.vregisters[0] as u16;
                return;
            },
            0xC000 => {
                // CXNN - set VX to random byte ANDed with NN
                chip8.vregisters[x as usize] = nn & chip8.rng.gen::<u8>();
                return;
            },
            0xD000 => {
                // DXYN - draw sprite at VX, VY with N bytes of sprite data starting at I
                let vx = chip8.vregisters[x as usize];
                let vy = chip8.vregisters[y as usize];

                let mut heightbytes = chip8.opcode & 0x000F;
                if heightbytes == 0 {
                    heightbytes = 16;
                }

                for yline in 0..heightbytes {
                    let wy = (vy as u16 + yline) % 32;

                    let line = chip8.memory[(chip8.i + yline) as usize];

                    for xline in 0..8 {
                        let wx = (vx as u16 + xline) % 64;
                        if (line & 0x80) > 0 {
                            if (chip8.display[(wy + wx * 32) as usize]) == 1 {
                                chip8.vregisters[15] = 1;
                            }
                            chip8.display[(wy + wx * 32) as usize] ^= 1;
                        }
                    }
                }
                return;
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