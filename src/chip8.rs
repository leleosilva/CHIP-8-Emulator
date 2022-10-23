use crate::cpu::Cpu;

pub struct Chip8 {
    cpu: Cpu,
}

impl Chip8 {

    // Creating new instance of CHIP-8
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
        }
    }

    // Loads ROM using CPU method
    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.cpu.load_rom_in_memory(rom_data);
    }
}