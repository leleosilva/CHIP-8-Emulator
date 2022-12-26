mod chip8;
mod cpu;

use chip8::Chip8;
use cpu::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

const WINDOW_SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * WINDOW_SCALE;

use sdl2::VideoSubsystem;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowBuildError};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();

    // Checking if the command line arguments are correct (currently temporary)
    if args.len() != 2 {
        return Err(String::from("Path to ROM file not found"));
    }
    println!("{:?}", args);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Building window
    let window = if let Ok(w) = build_sdl_window(video_subsystem) {
        w
    } else {
        return Err(String::from("Could not build SDL2 window"));
    };
    
    // Building canvas
    let mut canvas = if let Ok(c) = window.into_canvas().build() {
        c
    } else {
        return Err(String::from("Could not initialize a canvas from the specified SDL2 window"));
    };

    canvas.clear();
    canvas.present();

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
                _ => (),
            }
        }

        chip8.run();
        
        if let Err(c) = draw_display(&chip8, &mut canvas) {
            return Err(c);
        }
    }

    Ok(())
}

fn build_sdl_window(video: VideoSubsystem) -> Result<Window, WindowBuildError> {
    video
        .window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
}

fn draw_display(chip8: &Chip8, canvas: &mut Canvas<Window>) -> Result<(), String>{
    
    // Clear canvas using black color
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let chip8_display = chip8.get_display();

    // Draw color is set to white
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Iterating through each display pixel. If pixel is true, it should be drawn
    for (idx, pixel) in chip8_display.iter().enumerate() {
        if *pixel {
            let x_coord = (idx % DISPLAY_WIDTH) as u32;
            let y_coord = (idx / DISPLAY_WIDTH) as u32;

            let rect = Rect::new(
                (x_coord * WINDOW_SCALE) as i32,
                (y_coord * WINDOW_SCALE) as i32,
                WINDOW_SCALE,
                WINDOW_SCALE);

            canvas.fill_rect(rect)?
        }
    }
    canvas.present();
    Ok(())
}
