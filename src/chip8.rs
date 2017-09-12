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

    pub gfx: [u8; 64*32],
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub stack: [u16; 16],
    pub sp: usize,

    pub key: [u8; 16]
}

impl Default for Chip8 {
    fn default() -> Chip8 {
        Chip8 {
            opcode: 0,
            memory: [0; 4096],
            V: [0; 16],
            i: 0,
            pc: 0x200,

            gfx: [0; 64*32],
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
        Chip8 {
            .. Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.opcode = 0;
        self.memory = [0; 4096];
        self.V = [0; 16];
        self.i = 0;
        self.pc = 0x200;
        self.gfx = [0; 64*32];
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

        // TODO: split these off into organized groups
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x000F {
                    0x0000 => {/* 0x00E0 clear screen*/}
                    0x000E => {/* 0x00EE returns from subroutine */}
                    _ => not_implemented(self.opcode as usize, self.pc)
                }
            }
            0x0004 => {
                if self.V[(self.opcode & 0x00FF0) as usize >> 4] > (0xFF - self.V[(self.opcode & 0x0F00) as usize >> 8]) {
                    self.V[0xF] = 1; // carry
                } else {
                    self.V[0xF] = 0;
                }
                self.V[(self.opcode & 0x0F00) as usize >> 8] += self.V[(self.opcode & 0x00F0) as usize >> 4];
                self.pc += 2;
            }
            0x0033 => {
                self.memory[self.i as usize] = self.V[(self.opcode & 0x0F00) as usize >> 8] / 100;
                self.memory[self.i + 1 as usize] = (self.V[(self.opcode & 0x0F00) as usize >> 8] / 10) % 10;
                self.memory[self.i + 2 as usize] = (self.V[(self.opcode & 0x0F00) as usize >> 8] % 100) % 10;
                self.pc += 2;
            }
            0x2000 => {
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
                self.pc = (self.opcode & 0x0FFF) as usize;
            }
            0xA000 => {
                self.i = (self.opcode & 0x0FFF) as usize;
                self.pc += 2;
            }
            _ => not_implemented(self.opcode as usize, self.pc)
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!")
            }
            self.sound_timer -= 1;
        }
    }

    fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);        
    }
}

fn not_implemented(op: usize, pc: usize) {
        println!("Unknown opcode: op: {:x}, pc: {:x}", op, pc)
    }