use std::{fs::File, io::Read, panic};

use crate::Chip8;

impl Chip8 {
    pub fn load_rom(&mut self, path: &str) {
        let mut rom = match File::open(path) {
            Ok(it) => it,
            Err(e) => panic!("Failed to load rom at {} with error: {}", path, e),
        };
        let mut buffer = Vec::new();
        match rom.read_to_end(&mut buffer) {
            Ok(_) => (),
            Err(e) => panic!("Failed to read rom at {} with error: {}", path, e),
        }
        for (index, &byte) in buffer.iter().enumerate() {
            if 0x200 + index < 4096 {
                self.memory[0x200 + index] = byte;
            }
        }
        println!("Loaded rom from {}", path)
    }
}
