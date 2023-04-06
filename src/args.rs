use clap::{Parser};


/// CHIP-8 Emulator
#[derive(Parser, Debug)]

pub struct Chip8Args {
    /// path to ROM file
    pub rom: String,
}