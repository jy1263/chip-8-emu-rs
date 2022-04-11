mod chip8;
mod opcode_parser;
mod fstools;
mod input;
mod audio;
mod args;

use std::sync::{Arc, RwLock};

use crate::args::Rgb;
use crate::fstools::get_file_as_byte_vec;
use crate::chip8::Chip8;
use crate::input::parse_input;

#[macro_use]
extern crate savefile_derive;
extern crate savefile;

extern crate glium;

fn main() {

    // args
    let flags = crate::args::parse_args();

    // setup speed
    // devide 2 as fetch and decode is on same loop
    let runhz:u64 = flags.hz;
    let delay:u64 = 1000/runhz;
    let satisfiedruntimes: u64 = (1000/60)/delay;

    // setup cpu instance
    let mut chip8inst = Chip8::new();
    chip8inst.load_program(&get_file_as_byte_vec(flags.rom_path.as_str()));
    chip8inst.display = [flags.invert_colors; 2048];
    let chip8arc = Arc::new(RwLock::new(chip8inst));

    let loopchip8 = chip8arc.clone();
    std::thread::spawn(move || {
        let beeper = crate::audio::Beeper::new(flags.vol);
        let beeperexist = beeper.is_ok();
        if !beeperexist {
            println!("Could not initialize audio!");
        }

        let mut runtimes = 0;
        loop {
            let next_frame_time = std::time::Instant::now() + std::time::Duration::from_millis(delay);
            
            // timer stuff
            if runtimes >= satisfiedruntimes {
                if loopchip8.read().unwrap().delay_timer > 0 {
                    loopchip8.write().unwrap().delay_timer -= 1;
                }
                if loopchip8.read().unwrap().sound_timer > 0 {
                    if beeperexist {
                        beeper.as_ref().unwrap().play();
                    }
                    loopchip8.write().unwrap().sound_timer -= 1;
                }
                else if beeperexist {
                    beeper.as_ref().unwrap().pause();
                }

                runtimes = 0;
            }
            runtimes += 1;


            // cycle cpu
            loopchip8.write().unwrap().single_cycle();

            if next_frame_time > std::time::Instant::now() {
                std::thread::sleep(next_frame_time - std::time::Instant::now());
            }
        }
    });

    // setup opengl
    use glium::glutin;
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let eventloopchip8 = chip8arc.clone();
    let mut last_next_frame_time = std::time::Instant::now();

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        
        if last_next_frame_time <= std::time::Instant::now() {
            render_texture_to_target(&eventloopchip8.read().unwrap().display, &display, &flags.fg, &flags.bg);
            last_next_frame_time = next_frame_time;
        }

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                    // println!("{:?}", input.virtual_keycode.unwrap());
                    parse_input(input, &mut eventloopchip8.write().unwrap(), &flags);
                },
                _ => return,
            },
            _ => (),
        }
    });
}

fn render_texture_to_target(dispmem: &[u8; 2048], display: &glium::Display, fg: &Rgb, bg: &Rgb) {
    use crate::glium::Surface;

    let mut disptexturevec = vec![vec![(bg.r, bg.g, bg.b); 64]; 32];
    for i in  0..dispmem.len() {
        if dispmem[i] == 1 {
            disptexturevec[31 - (i % 32)][i / 32] = (fg.r, fg.g, fg.b);
        }
    }
    let texture = glium::Texture2d::new(display, disptexturevec).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
    target.finish().unwrap();
}