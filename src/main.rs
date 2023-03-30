mod chip8;
mod cpu;
mod drivers;

use chip8::Chip8;
use drivers::DisplayDriver;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();

    // Checking if the command line arguments are correct (currently temporary)
    if args.len() != 2 {
        return Err(String::from("Path to ROM file not found"));
    }

    let sdl_context = sdl2::init()?;

    let mut display_driver = DisplayDriver::new(&sdl_context, None, None)?;
    

    // Creating event pump
    let mut event_pump = sdl_context.event_pump()?;

    // Reading ROM file
    let rom_data;
    match std::fs::read(&args[1]) {
        Ok(data) => rom_data = data,
        Err(_e) => return Err(format!(".ch8 file could not be found or read on path '{}'", &args[1])),
    };
    
    let mut chip8 = Chip8::new();
    
    chip8.load_rom(&rom_data);

    'emulation_cycle: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'emulation_cycle;
                },
                Event::KeyDown {keycode: Some(key), ..} => {
                    if let Some(k) = keycode_to_keypad(key) {
                        chip8.press_key(k);
                    }
                },
                Event::KeyUp {keycode: Some(key), ..} => {
                    if let Some(k) = keycode_to_keypad(key) {
                        chip8.release_key(k);
                    }
                },
                _ => (),
            }
        }

        if chip8.tick_period.elapsed() >= std::time::Duration::from_micros(2000) {
            chip8.run();
            if let Err(c) = display_driver.draw_display(chip8.get_display()) {
                return Err(c);
            }
            chip8.tick_period = std::time::Instant::now();
        }
    }
    Ok(())
}


fn keycode_to_keypad(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 =>    Some(0x1),
        Keycode::Num2 =>    Some(0x2),
        Keycode::Num3 =>    Some(0x3),
        Keycode::Num4 =>    Some(0xC),
        Keycode::Q =>       Some(0x4),
        Keycode::W =>       Some(0x5),
        Keycode::E =>       Some(0x6),
        Keycode::R =>       Some(0xD),
        Keycode::A =>       Some(0x7),
        Keycode::S =>       Some(0x8),
        Keycode::D =>       Some(0x9),
        Keycode::F =>       Some(0xE),
        Keycode::Z =>       Some(0xA),
        Keycode::X =>       Some(0x0),
        Keycode::C =>       Some(0xB),
        Keycode::V =>       Some(0xF),
        _ =>                None,
    }
}