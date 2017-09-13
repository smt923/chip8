#![allow(dead_code)] // sorry - TODO: remove later
extern crate sdl2;
extern crate rand;

mod chip8;
mod display;
mod input;

use chip8::Chip8;
use display::Display;
use input::Input;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please give the path to the program to load");
        std::process::exit(1)
    }
    let mut chip8 = Chip8::new();
    let mut prog: Vec<u8> = vec![];

    let mut f = match File::open(&args[1]) {
        Ok(file) => file,
        Err(why) => panic!(why),
    };
    f.read_to_end(&mut prog).expect("Could not read program");

    chip8.load(prog);
    chip8.display.clear();
    chip8.display.canvas.present();
    let mut event_pump = chip8.display.ctx.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: key, ..} => {
                    chip8.input.press(key);
                    continue
                },
                Event::KeyUp {keycode: key, ..} => {
                    chip8.input.release(key);
                    continue
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        chip8.emulate_cycle();
        if chip8.draw_flag {
            chip8.display.draw(&chip8.gfx);
            chip8.display.clear();
            chip8.draw_flag = false;
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }
}
