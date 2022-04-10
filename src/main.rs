mod chip8;
mod opcode_parser;
mod fstools;
mod input;
mod args;

use crate::args::Rgb;
use crate::fstools::get_file_as_byte_vec;
use crate::chip8::Chip8;
use crate::input::parse_input;

extern crate glium;

fn main() {
    // args
    let flags = crate::args::parse_args();

    // setup speed
    // devide 2 as fetch and decode is on same loop
    let runhz:u64 = flags.hz/2;
    let delay:u64 = 1000/runhz;
    let satisfiedruntimes: u64 = (1000/60)/delay;

    // setup cpu instance
    let mut chip8inst = Chip8::new();
    chip8inst.load_program(&get_file_as_byte_vec(flags.rom_path.as_str()));
    chip8inst.display = [flags.invert_colors; 2048];

    // setup opengl
    let mut runtimes = 0;

    use glium::glutin;
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_millis(delay);

        // timer stuff
        if runtimes >= satisfiedruntimes {
            render_texture_to_target(&chip8inst.display, &display, &flags.fg, &flags.bg);
            if chip8inst.delay_timer > 0 {
                chip8inst.delay_timer -= 1;
            }
            if chip8inst.sound_timer > 0 {
                chip8inst.sound_timer -= 1;
            }
            runtimes = 0;
        }
        runtimes += 1;

        // cycle cpu
        chip8inst.single_cycle();
        
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                    // println!("{:?}", input.virtual_keycode.unwrap());
                    parse_input(input, &mut chip8inst);
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