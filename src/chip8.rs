use crate::cpu::Cpu;
use std::time;


pub struct Chip8 {
    cpu: Cpu,
    pub tick_period: time::Instant,
}

impl Chip8 {

    // Creating new instance of CHIP-8
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            tick_period: std::time::Instant::now(),
        }
    }

    // Loads ROM using CPU method
    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), String> {
        self.cpu.load_rom_in_memory(rom_data)
    }

    // Runs CHIP-8
    pub fn run(&mut self) {
        self.cpu.run();
    }

    // Returns the display using the CPU method
    pub fn get_display(&self) -> &[bool] {
        self.cpu.get_display()
    }

    // Returns the beep sound flag
    pub fn get_beep_state(&self) -> bool {
        self.cpu.get_beep_state()
    }

    // Returns the display update flag
    pub fn get_display_state(&self) -> bool {
        self.cpu.get_display_state()
    }

    // Sets key of chosen index as pressed
    pub fn press_key(&mut self, keypad_idx: usize) {
        self.cpu.set_key(keypad_idx, true);
    }

    // Sets key of chosen index as released
    pub fn release_key(&mut self, keypad_idx: usize) {
        self.cpu.set_key(keypad_idx, false);
    }

}