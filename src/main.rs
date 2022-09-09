mod chip8;
mod cpu;

use chip8::Chip8;

fn main() {
    
    // Reading ROM file
    let rom_data = std::fs::read("../roms/UFO").expect("File not read correctly");
    
    let mut chip8 = Chip8::new();
    
    chip8.cpu.load_rom(&rom_data);
}
