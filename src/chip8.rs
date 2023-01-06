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

    // Returns the display using the CPU method
    pub fn get_display(&self) -> &[bool] {
        self.cpu.get_display()
    }

    // Runs CHIP-8
    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn press_key(&mut self, keypad_idx: usize) {
        self.cpu.set_key(keypad_idx, true);
    }

    pub fn release_key(&mut self, keypad_idx: usize) {
        self.cpu.set_key(keypad_idx, false);
    }
}