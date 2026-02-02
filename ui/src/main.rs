use chip8_core;

fn main() {
    let mut chip8 = chip8_core::new();
    chip8.load_rom("test_rom.ch8");
    for _ in 0..60 {
        let opcode = chip8.fetch();
        let (op, x, y, n, nn, nnn) = chip8.decode(opcode);
        chip8.execute(op, x, y, n, nn, nnn);
    }
    println!("completed run")
}
