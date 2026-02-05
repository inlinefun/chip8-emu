use crate::Chip8;

impl Chip8 {
    pub(crate) fn clear_screen(&mut self) {
        self.display.fill(false);
    }
    pub(crate) fn jump_to(&mut self, value: u16) {
        self.pc = value;
    }
    pub(crate) fn set_register(&mut self, register: u16, value: u16) {
        self.v[register as usize] = value;
    }
    pub(crate) fn add_value_to_register(&mut self, register: u16, value: u16) {
        self.v[register as usize] = self.v[register as usize].wrapping_add(value);
    }
    pub(crate) fn set_index_register(&mut self, value: u16) {
        self.i = value;
    }
    pub(crate) fn draw(&mut self, x: u16, y: u16, n: u16) {
        let x_coord = self.v[x as usize] % 64; // width
        let y_coord = self.v[y as usize] % 32; // height
        // set vF to 0
        self.v[0xF] = 0;
        for i in 0..n {
            let nth_byte = self.memory[(self.i + i) as usize];
            for bit in 0..8 {
                if nth_byte & (0x80 >> bit) != 0 {
                    let current_x_coord = x_coord + bit;
                    let current_y_coord = y_coord + i;
                    let index = current_x_coord + (current_y_coord * 64);
                    if self.display[index as usize] {
                        self.v[0xF as usize] = 1;
                    }
                    self.display[index as usize] ^= true;
                }
            }
        }
    }
    pub(crate) fn call_subroutine(&mut self, value: u16) {
        self.stack.push(self.pc);
        self.pc = value;
    }
    pub(crate) fn return_subroutine(&mut self) {
        match self.stack.pop() {
            Some(it) => self.pc = it,
            None => eprintln!("tried to return subroutine on empty stack"),
        }
    }
    // id?? idk
    pub(crate) fn skip_if_value_equal(&mut self, register_id: u16, value: u16) {
        if self.v[register_id as usize] == value {
            self.pc += 2;
        }
    }
    pub(crate) fn skip_if_value_not_equal(&mut self, register_id: u16, value: u16) {
        if self.v[register_id as usize] != value {
            self.pc += 2;
        }
    }
    pub(crate) fn skip_if_register_equals(&mut self, register_1: u16, register2: u16) {
        if self.v[register_1 as usize] == self.v[register2 as usize] {
            self.pc += 2;
        }
    }
    pub(crate) fn skip_if_register_not_equals(&mut self, register_1: u16, register2: u16) {
        if self.v[register_1 as usize] != self.v[register2 as usize] {
            self.pc += 2;
        }
    }
    pub(crate) fn jump_with_offset(&mut self, value: u16) {
        self.pc = self.v[0] + value;
    }
    pub(crate) fn random_number(&mut self, register: u16, value: u16) {
        let random = rand::random_range(0..value);
        self.v[register as usize] = random & value;
    }
    pub(crate) fn skip_if_key(&mut self, x: u16) {
        todo!("implement key {} press", x)
    }
    pub(crate) fn set_to_delay_timer(&mut self, register: u16) {
        self.v[register as usize] = self.delay_timer as u16;
    }
    pub(crate) fn set_delay_timer(&mut self, register: u16) {
        self.delay_timer = self.v[register as usize];
    }
    pub(crate) fn set_sound_timer(&mut self, register: u16) {
        self.sound_timer = self.v[register as usize];
    }
    pub(crate) fn add_to_index(&mut self, register: u16) {
        self.i += self.v[register as usize];
    }
    pub(crate) fn get_key(&mut self, register: u16) {
        // fetch increments by 2
        // this decrements by 2
        // therefore halts the program from continuing further
        // once a key is pressed, the key's value is updated in the v[register]
        // note: delay_timer and sound_timer should be decremented while this is happening
        self.pc -= 2;
        todo!(
            "implement get key and update value in register {}",
            register
        )
    }
    pub(crate) fn font_character(&mut self, register: u16) {
        todo!(
            "fetch font of the hexadecimal character at register {}",
            register
        )
        // and then set self.i to it
    }
    pub(crate) fn decimal_conversion(&mut self, register: u16) {
        let vx = self.v[register as usize];
        let a = (vx & 0x0F00) >> 8;
        let b = (vx & 0x00F0) >> 4;
        let c = vx & 0x000F;
        let base_address = self.i;
        self.memory[base_address as usize] = a as u8;
        self.memory[(base_address + 1) as usize] = b as u8;
        self.memory[(base_address + 2) as usize] = c as u8;
    }
    pub(crate) fn store_to_memory(&mut self, register: u16) {
        let base_address = self.i;
        // +1 so that the register itself is included as well
        for i in 0..(register + 1) {
            self.v[i as usize] = self.memory[(base_address + i) as usize] as u16;
        }
    }
    pub(crate) fn load_from_memory(&mut self, register: u16) {
        let base_address = self.i;
        // +1 so that the register itself is included as well
        for i in 0..(register + 1) {
            self.memory[(base_address + i) as usize] = self.v[i as usize] as u8;
        }
    }
    pub(crate) fn handle_arithemetics(&mut self, x: u16, y: u16, identifier: u16) {
        match identifier {
            0 => self.v[x as usize] = self.v[y as usize],
            1 => self.v[x as usize] = self.v[x as usize] | self.v[y as usize],
            2 => self.v[x as usize] = self.v[x as usize] & self.v[y as usize],
            3 => self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize],
            4 => {
                let result = self.v[x as usize] + self.v[y as usize];
                if result > 255 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                // this is so stupid lmao
                self.v[x as usize] = (result as u8) as u16;
            }
            5 => {
                if self.v[x as usize] > self.v[y as usize] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x as usize] =
                    (self.v[x as usize] as u8).wrapping_sub(self.v[y as usize] as u8) as u16;
            }
            7 => {
                if self.v[y as usize] > self.v[x as usize] {
                    self.v[0xF] = 1;
                }
                self.v[x as usize] =
                    (self.v[y as usize] as u8).wrapping_sub(self.v[x as usize] as u8) as u16;
            }
            6 => {
                let value = self.v[y as usize];
                self.v[x as usize] = value >> 1;
            }
            0xE => {
                let value = self.v[y as usize];
                self.v[x as usize] = value << 1;
            }
            _ => {
                panic!(
                    "unimplemented arithemetic operation with identifier {}. \n instruction: 8{:X}{:X}{:X}",
                    identifier, x, y, identifier
                )
            }
        }
    }
}
