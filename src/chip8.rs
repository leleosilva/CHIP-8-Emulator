use crate::cpu::Cpu;

pub struct Chip8 {
    cpu: Cpu,
}

impl Chip8 {

    // Creating new instance of CHIP-8
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
        }
    }
}