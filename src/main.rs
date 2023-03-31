mod chip8;
mod cpu;
mod drivers;

use chip8::Chip8;
use drivers::{DisplayDriver, KeypadDriver};

const CHIP8_RATE: u64 = 2000;

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();

    // Checking if the command line arguments are correct (currently temporary)
    if args.len() != 2 {
        return Err(String::from("Path to ROM file not found"));
    }

    let sdl_context = sdl2::init()?;

    let mut display_driver = DisplayDriver::new(&sdl_context, None, None)?;
    let mut keypad_driver = KeypadDriver::new(&sdl_context)?;

    // Reading ROM file
    let rom_data;
    match std::fs::read(&args[1]) {
        Ok(data) => rom_data = data,
        Err(_e) => return Err(format!(".ch8 file could not be found or read on path '{}'", &args[1])),
    };
    
    let mut chip8 = Chip8::new();
    
    chip8.load_rom(&rom_data);

    // Keep the CHIP-8 running as long as a quit event 'Err(())' has not been received
    while let Ok(k) = keypad_driver.poll_event() {
        
        // Key press/release event
        if let Some(k) = k {
            if keypad_driver.key_pressed {
                chip8.press_key(k);
            } else {
                chip8.release_key(k);
            }
        }
        
        // Ensures that CHIP-8 runs at a rate of 500Hz (1s / 500 = 2000 microseconds)
        if chip8.tick_period.elapsed() >= std::time::Duration::from_micros(CHIP8_RATE) {
            chip8.run();
            
            // Updates the display at a rate of 60Hz
            if chip8.get_display_state() {
                if let Err(c) = display_driver.draw_display(chip8.get_display()) {
                    return Err(c);
                }
            }
            
            // Beeps at a rate of 60Hz
            if chip8.get_beep_state() {
                println!("Beep!");
            }
            
            chip8.tick_period = std::time::Instant::now();
        }
    }
    Ok(())
}