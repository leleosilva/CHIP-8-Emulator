use sdl2::VideoSubsystem;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowBuildError};
use sdl2::pixels::Color;
use sdl2::render::Canvas;

use crate::cpu::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

const WINDOW_SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * WINDOW_SCALE;


pub struct DisplayDriver {
    canvas: Canvas<Window>,
    bg_color: Color,
    main_color: Color,
}

impl DisplayDriver {

    pub fn new(sdl_context: &sdl2::Sdl, bg_color: Option<Color>, main_color: Option<Color>) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;

        // Building window
        let window = if let Ok(w) = Self::build_sdl_window(video_subsystem) {
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

        Ok(Self {
            canvas,
            bg_color: bg_color.unwrap_or(Color::RGB(0, 0, 0)),
            main_color: main_color.unwrap_or(Color::RGB(255, 255, 255)),
        })
    }


    fn build_sdl_window(video: VideoSubsystem) -> Result<Window, WindowBuildError> {
        video
            .window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
    }


    pub fn draw_display(&mut self, chip8_display: &[bool]) -> Result<(), String>{
        
        // Clear canvas using black color
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();

        // Draw color is set to white
        self.canvas.set_draw_color(self.main_color);

        // Iterating through each display pixel. If pixel is true, it should be drawn
        for (idx, pixel) in chip8_display.iter().enumerate() {
            if *pixel {
                let x_coord = (idx % DISPLAY_WIDTH) as u32;
                let y_coord = (idx / DISPLAY_WIDTH) as u32;

                let rect = Rect::new(
                    (x_coord * WINDOW_SCALE) as i32,
                    (y_coord * WINDOW_SCALE) as i32,
                    WINDOW_SCALE,
                    WINDOW_SCALE
                );

                self.canvas.fill_rect(rect)?
            }
        }
        self.canvas.present();
        Ok(())
    }
}