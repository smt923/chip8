use std::{thread, time};

pub struct Chip8 {
    /*
        0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
        0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
        0x200-0xFFF - Program ROM and work RAM
    */
    pub opcode: u16,
    pub memory: [u8; 4096],
    pub V: [u8; 16],
    pub i: usize,
    pub pc: usize,

    pub gfx: [u8; 64 * 32],
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub stack: [u16; 16],
    pub sp: usize,

    pub key: [u8; 16],
}

impl Default for Chip8 {
    fn default() -> Chip8 {
        Chip8 {
            opcode: 0,
            memory: [0; 4096],
            V: [0; 16],
            i: 0,
            pc: 0x200,

            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,

            stack: [0; 16],
            sp: 0,

            key: [0; 16],
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
        self.gfx = [0; 64 * 32];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack = [0; 16];
        self.sp = 0;
        self.key = [0; 16];
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
        println!("{}: 0x{:04X}", self.pc, self.opcode);
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
        thread::sleep(time::Duration::from_millis(100))
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
            0x0000 => { /*self.display.clear()*/ }
            0x000E => {
                self.sp -= 1;
                self.pc = self.stack[self.sp] as usize;
            }
            _ => not_implemented(self.opcode as usize, self.pc),
        }
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
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn x4(&mut self) {
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn x5(&mut self) {
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn x6(&mut self) {
        self.V[self.op_x() as usize] = self.op_nn();
        self.pc += 2;
    }
    fn x7(&mut self) {
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn x8(&mut self) {
        match self.opcode & 0x000F {
            0 => self.V[self.op_x()] = self.V[self.op_y()],
            1 => self.V[self.op_x()] |= self.V[self.op_y()],
            2 => self.V[self.op_x()] &= self.V[self.op_y()],
            3 => self.V[self.op_x()] ^= self.V[self.op_y()],
            4 => {
                self.V[self.op_x()] += self.V[self.op_y()];
                self.V[15] = if self.V[self.op_x()] < self.V[self.op_y()] {
                    1
                } else {
                    0
                };
            }
            _ => not_implemented(self.opcode as usize, self.pc),
        }
    }
    fn x9(&mut self) {
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn xA(&mut self) {
        self.i = self.op_nnn() as usize;
        self.pc += 2;
    }
    fn xB(&mut self) {
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn xC(&mut self) {
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn xD(&mut self) {
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn xE(&mut self) {
        not_implemented(self.opcode as usize, self.pc);
        self.pc += 2;
    }
    fn xF(&mut self) {
        match self.opcode & 0x00FF {
            0x33 => {
                self.memory[self.i] = self.V[self.op_x()] / 100;
                self.memory[self.i + 1] = (self.V[self.op_x()] / 10) % 10;
                self.memory[self.i + 2] = (self.V[self.op_x()] % 100) % 10;
            }
            _ => not_implemented(self.opcode as usize, self.pc),
        }
        self.pc += 2;
    }
}

fn not_implemented(op: usize, pc: usize) {
    println!(
        "opcode not yet implemented: op: 0x{:04X}, pc: {:08X}",
        op,
        pc
    )
}

const FONTSET: [u8; 80] = [
    0xF0,
    0x90,
    0x90,
    0x90,
    0xF0,
    0x20,
    0x60,
    0x20,
    0x20,
    0x70,
    0xF0,
    0x10,
    0xF0,
    0x80,
    0xF0,
    0xF0,
    0x10,
    0xF0,
    0x10,
    0xF0,
    0x90,
    0x90,
    0xF0,
    0x10,
    0x10,
    0xF0,
    0x80,
    0xF0,
    0x10,
    0xF0,
    0xF0,
    0x80,
    0xF0,
    0x90,
    0xF0,
    0xF0,
    0x10,
    0x20,
    0x40,
    0x40,
    0xF0,
    0x90,
    0xF0,
    0x90,
    0xF0,
    0xF0,
    0x90,
    0xF0,
    0x10,
    0xF0,
    0xF0,
    0x90,
    0xF0,
    0x90,
    0x90,
    0xE0,
    0x90,
    0xE0,
    0x90,
    0xE0,
    0xF0,
    0x80,
    0x80,
    0x80,
    0xF0,
    0xE0,
    0x90,
    0x90,
    0x90,
    0xE0,
    0xF0,
    0x80,
    0xF0,
    0x80,
    0xF0,
    0xF0,
    0x80,
    0xF0,
    0x80,
    0x80,
];
