use sdl2::keyboard::Keycode;

pub struct Input {
    pub keys: [bool; 16],
}

impl Input {
    pub fn new() -> Input {
        Input { keys: [false; 16] }
    }

    pub fn pressed(&mut self, index: usize) -> bool {
        self.keys[index]
    }

    fn set_key(&mut self, index: usize, state: bool) {
        self.keys[index] = state;
    }

    pub fn press(&mut self, key: Option<Keycode>) {
        if key.is_some() {
            match key.unwrap() {
                Keycode::Num1 => self.set_key(0x1, true),
                Keycode::Num2 => self.set_key(0x2, true),
                Keycode::Num3 => self.set_key(0x3, true),
                Keycode::Num4 => self.set_key(0xc, true),
                Keycode::Q    => self.set_key(0x4, true),
                Keycode::W    => self.set_key(0x5, true),
                Keycode::E    => self.set_key(0x6, true),
                Keycode::R    => self.set_key(0xD, true),
                Keycode::A    => self.set_key(0x7, true),
                Keycode::S    => self.set_key(0x8, true),
                Keycode::D    => self.set_key(0x9, true),
                Keycode::F    => self.set_key(0xE, true),
                Keycode::Z    => self.set_key(0xA, true),
                Keycode::X    => self.set_key(0x0, true),
                Keycode::C    => self.set_key(0xB, true),
                Keycode::V    => self.set_key(0xF, true),
                _ => ()
            }
        }
    }

    pub fn release(&mut self, key: Option<Keycode>) {
        if key.is_some() {
            match key.unwrap() {
                Keycode::Num1 => self.set_key(0x1, false),
                Keycode::Num2 => self.set_key(0x2, false),
                Keycode::Num3 => self.set_key(0x3, false),
                Keycode::Num4 => self.set_key(0xc, false),
                Keycode::Q    => self.set_key(0x4, false),
                Keycode::W    => self.set_key(0x5, false),
                Keycode::E    => self.set_key(0x6, false),
                Keycode::R    => self.set_key(0xD, false),
                Keycode::A    => self.set_key(0x7, false),
                Keycode::S    => self.set_key(0x8, false),
                Keycode::D    => self.set_key(0x9, false),
                Keycode::F    => self.set_key(0xE, false),
                Keycode::Z    => self.set_key(0xA, false),
                Keycode::X    => self.set_key(0x0, false),
                Keycode::C    => self.set_key(0xB, false),
                Keycode::V    => self.set_key(0xF, false),
                _ => ()
            }
        }
    }
}