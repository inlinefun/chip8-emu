mod instructions;
pub mod util;

pub struct Chip8 {
    // 4 kilobytes of RAM
    memory: [u8; 4096],
    // 64 x 32 pixels monochrome display
    pub display: [bool; 64 * 32],
    // program counter
    pc: u16,
    // index register
    i: u16,
    // A stack for 16-bit addresses
    stack: [u16; 16],
    // 8-bit delay timer
    delay_timer: u8,
    // 8-bit sound timer
    sound_timer: u8,
    // 16 8-bit (one byte) general-purpose variable registers
    // made 16 cuz yes
    v: [u16; 16],
}

pub fn new() -> Chip8 {
    return Chip8 {
        memory: [0; 4096],
        display: [false; 64 * 32],
        pc: 0x200,
        i: 0,
        stack: [0; 16],
        delay_timer: 0,
        sound_timer: 0,
        v: [0; 16],
    };
}

impl Chip8 {
    pub fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        let opcode = (hi << 8) | lo;
        self.pc += 2;
        return opcode;
    }
    pub fn decode(&mut self, opcode: u16) -> (u16, u16, u16, u16, u16, u16) {
        let op = (opcode & 0xF000) >> 12;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let n = opcode & 0x000F;
        let nn = opcode & 0x00FF;
        let nnn = opcode & 0x0FFF;
        return (op, x, y, n, nn, nnn);
    }
    pub fn execute(&mut self, op: u16, x: u16, y: u16, n: u16, nn: u16, nnn: u16) {
        match (op, x, y, n) {
            // 00E0, clear the screen
            (0, 0, 0xE, 0) => self.clear_screen(),
            // 1nnn, jump
            (1, _, _, _) => self.jump_to(nnn),
            // 6xnn, set register 'x' to 'nn'
            (6, _, _, _) => self.set_register(x, nn),
            // 7xnn, add 'nn' to register 'x'
            (7, _, _, _) => self.add_value_to_register(x, nn),
            // annn, set index register to 'nnn'
            (0xA, _, _, _) => self.set_index_register(nnn),
            // dxyn, draw
            (0xD, _, _, _) => self.draw(x, y, n),
            (_, _, _, _) => {
                eprintln!("invalid operation.");
                eprintln!(
                    "op: {:X}, x: {:X}, y: {:X}, n: {:X}, nn: {:X}, nnn: {:X}",
                    op, x, y, n, nn, nnn
                );
                panic!("operation {} not implemented", op)
            }
        }
    }
    pub fn advance_game_loop(&mut self) {
        let opcode = self.fetch();
        let (op, x, y, n, nn, nnn) = self.decode(opcode);
        self.execute(op, x, y, n, nn, nnn);
    }
}
