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
        self.v[register as usize] = self.v[register as usize].wrapping_add(value)
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
}
