#![allow(dead_code)] // sorry - TODO: remove later
mod chip8;

use chip8::Chip8;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please give the path to the program to load");
        std::process::exit(1)
    }
    println!("chip8 emulator");
    let mut chip8 = Chip8::new();

    let mut prog: Vec<u8> = vec![];

    let mut f = match File::open(&args[1]) {
        Ok(file) => file,
        Err(why) => panic!(why),
    };
    f.read_to_end(&mut prog).expect("Could not read program");

    chip8.load(prog);
    loop {
        chip8.emulate_cycle();
    }
}
