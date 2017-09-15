use display::Display;
use input::Input;

use std::{thread, time};
use rand;
use rand::Rng;

pub const SCREEN_W: usize = 64;
pub const SCREEN_H: usize = 32;

/// This represents the 'core' and cpu of the system, along with some
/// fields for easy access to useful things we need to use elsewhere.
///
/// 'gfx' here is an internal representation of the graphics state, but
/// is not directly outputted to anything, currently, SDL takes a copy of this
/// and draws a scaled up verison using it.
///
/// Memory Reference:
/// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
/// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
/// 0x200-0xFFF - Program ROM and work RAM
pub struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    V: [u8; 16],
    i: usize,
    pc: usize,

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    sp: usize,
    pub gfx: [u8; SCREEN_W*SCREEN_H],
    pub draw_flag: bool,

    pub input: Input,
    pub display: Display,
}

impl Default for Chip8 {
    fn default() -> Chip8 {
        Chip8 {
            opcode: 0,
            memory: [0; 4096],
            V: [0; 16],
            i: 0,
            pc: 0x200,

            delay_timer: 0,
            sound_timer: 0,

            stack: [0; 16],
            sp: 0,
            gfx: [0; SCREEN_W*SCREEN_H],
            draw_flag: false,

            input: Input::new(),
            display: Display::new(),
        }
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip = Chip8 {
            ..Default::default()
        };
        for i in 0..80 {
            chip.memory[i] = FONTSET[i];
        }
        chip
    }

    pub fn reset(&mut self) {
        self.opcode = 0;
        self.memory = [0; 4096];
        self.V = [0; 16];
        self.i = 0;
        self.pc = 0x200;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack = [0; 16];
        self.sp = 0;
    }

    pub fn debug_memory(&self) {
        let mut i = 0;
        while i < self.memory.len() {
            print!("{:?} ", self.memory[i]);
            i += 1
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let mut i = 0;
        while i < program.len() {
            self.memory[i + 512] = program[i];
            i += 1
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch_opcode();
        // debug print to get a rough idea of what's going on
        //println!("{}: 0x{:04X}", self.pc, self.opcode);
        self.opcodes();

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!")
            }
            self.sound_timer -= 1;
        }
        //thread::sleep(time::Duration::from_millis(100))
    }

    fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    // Based off https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
    fn op_nnn(&self) -> u16 {
        self.opcode & 0x0FFF
    }
    fn op_nn(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }
    fn op_n(&self) -> u8 {
        (self.opcode & 0x000F) as u8
    }
    fn op_x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }
    fn op_y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4) as usize
    }

    pub fn opcodes(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => self.x0(),
            0x1000 => self.x1(),
            0x2000 => self.x2(),
            0x3000 => self.x3(),
            0x4000 => self.x4(),
            0x5000 => self.x5(),
            0x6000 => self.x6(),
            0x7000 => self.x7(),
            0x8000 => self.x8(),
            0x9000 => self.x9(),
            0xA000 => self.xA(),
            0xB000 => self.xB(),
            0xC000 => self.xC(),
            0xD000 => self.xD(),
            0xE000 => self.xE(),
            0xF000 => self.xF(),
            _ => not_implemented(self.opcode as usize, self.pc),
        }
    }

    fn x0(&mut self) {
        match self.opcode & 0x000F {
            0x0000 => {
                self.clear_gfx();
                self.display.clear();
                self.draw_flag = true;
            }
            0x000E => {
                self.sp -= 1;
                self.pc = self.stack[self.sp] as usize;
            }
            _ => not_implemented(self.opcode as usize, self.pc),
        }
        self.pc += 2;
    }
    fn x1(&mut self) {
        self.pc = self.op_nnn() as usize;
    }
    fn x2(&mut self) {
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = self.op_nnn() as usize;
    }
    fn x3(&mut self) {
        if self.V[self.op_x()] == self.op_nn() {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }
    fn x4(&mut self) {
        if self.V[self.op_x()] != self.op_nn() {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }
    fn x5(&mut self) {
        if self.V[self.op_x()] == self.V[self.op_y()] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }
    fn x6(&mut self) {
        self.V[self.op_x()] = self.op_nn();
        self.pc += 2;
    }
    fn x7(&mut self) {
        self.V[self.op_x()] = self.V[self.op_x()].wrapping_add(self.op_nn());
        self.pc += 2;
    }
    fn x8(&mut self) {
        match self.opcode & 0x000F {
            0 => self.V[self.op_x()] = self.V[self.op_y()],
            1 => self.V[self.op_x()] |= self.V[self.op_y()],
            2 => self.V[self.op_x()] &= self.V[self.op_y()],
            3 => self.V[self.op_x()] ^= self.V[self.op_y()],
            4 => {
                let vx = self.V[self.op_x()];
                let vy = self.V[self.op_y()];
                let (val, flag) = vx.overflowing_add(vy);
                self.V[0xF] = if flag { 1 } else { 0 };
                self.V[self.op_x()] = val;
            }
            5 => {
                let vx = self.V[self.op_x()];
                let vy = self.V[self.op_y()];
                let (val, flag) = vx.overflowing_sub(vy);
                self.V[0xF] = if flag { 1 } else { 0 };
                self.V[self.op_x()] = val;
            }
            6 => {
                let vx = self.V[self.op_x()];
                let vy = self.V[self.op_y()];
                let (val, flag) = vx.overflowing_shr(vy as u32);
                self.V[0xF] = if flag { 1 } else { 0 };
                self.V[self.op_x()] = val;
            }
            7 => {
                let vx = self.V[self.op_x()];
                let vy = self.V[self.op_y()];
                let (val, flag) = vy.overflowing_sub(vx);
                self.V[0xF] = if flag { 1 } else { 0 };
                self.V[self.op_x()] = val;
            }
            0xE => {
                let vx = self.V[self.op_x()];
                let vy = self.V[self.op_y()];
                let (val, flag) = vx.overflowing_shl(vy as u32);
                self.V[0xF] = if flag { 1 } else { 0 };
                self.V[self.op_x()] = val;
            }
            _ => not_implemented(self.opcode as usize, self.pc),
        }
        self.pc += 2;
    }
    fn x9(&mut self) {
        if self.V[self.op_x()] != self.V[self.op_y()] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }
    fn xA(&mut self) {
        self.i = self.op_nnn() as usize;
        self.pc += 2;
    }
    fn xB(&mut self) {
        self.pc = (self.V[0x0] as u16 + self.op_nnn()) as usize;
    }
    fn xC(&mut self) {
        let mut n = rand::thread_rng().gen_range(0, 255);
        self.V[self.op_x()] = n & self.op_nn();
        self.pc += 2;
    }
    fn xD(&mut self) {
        let x = self.V[self.op_x()] as usize;
        let y = self.V[self.op_y()] as usize;
        let height = self.op_n();
        self.V[0xF] = 0;

        for yl in 0..height {
            let pixel = self.memory[self.i + yl as usize] as u16;
            for xl in 0..8 {
                if pixel & (0x80 >> xl) != 0 {
                    let mut idx = (x + xl + ((y + yl as usize) * SCREEN_W));
                    if idx >= 2047 {
                        idx = 2047
                    } else if idx <= 0 {
                        idx = 0
                    }
                    if self.gfx[idx] == 1 {
                        self.V[0xF] = 1;
                    }
                    self.gfx[idx] ^= 1;
                }
            }
        }
        self.draw_flag = true;
        self.pc += 2;
    }
    fn xE(&mut self) {
        let k = self.V[self.op_x()] as usize;
        self.pc += match self.opcode & 0x00FF {
            0x9E => if self.input.pressed(k) { 4 } else { 2 },
            0xA1 => if !self.input.pressed(k) { 4 } else { 2 },
            _    => 2
        }
    }
    fn xF(&mut self) {
        match self.opcode & 0x00FF {
            0x07 => { self.V[self.op_x()] = self.delay_timer; }
            0x0A => { self.get_keypress(); }
            0x15 => { self.delay_timer = self.V[self.op_x()] }
            0x18 => { self.sound_timer = self.V[self.op_x()] }
            0x1E => { self.i += self.V[self.op_x()] as usize }
            0x29 => { self.i = (self.V[self.op_x()] as usize) * 5; }
            0x33 => {
                self.memory[self.i] = self.V[self.op_x()] / 100;
                self.memory[self.i + 1] = (self.V[self.op_x()] / 10) % 10;
                self.memory[self.i + 2] = (self.V[self.op_x()] % 100) % 10;
            }
            0x55 => {
                for i in 0..self.op_x()+1 {
                    self.memory[self.i + i] = self.V[i]
                }
            }
            0x65 => {
                for i in 0..self.op_x()+1 {
                    self.V[i] = self.memory[self.i + i] 
                }
            }
            _ => not_implemented(self.opcode as usize, self.pc),
        }
        self.pc += 2;
    }

    fn get_keypress(&mut self) {
        for i in 0u8..16 {
                if self.input.pressed(i as usize) {
                    self.V[self.op_x()] = i;
                    break;
                }
            }
        self.pc -= 2;
    }

    pub fn clear_gfx(&mut self) {
        for i in 0..self.gfx.len() {
            self.gfx[i] = 0;
        }
    }

    pub fn get_internal_display(&self) -> &[u8] {
        &self.gfx
    }
}


fn not_implemented(op: usize, pc: usize) {
    println!(
        "opcode not yet implemented: op: 0x{:04X}, pc: {:08X}",
        op,
        pc
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
