use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct KeypadDriver {
    event_pump: sdl2::EventPump,
    pub key_pressed: bool,
}

impl KeypadDriver {

    // Creates new instance of the keypad driver
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            event_pump,
            key_pressed: false,
        })
    }

    // Polling events checking for Quit, KeyDown and KeyUp events
    pub fn poll_event(&mut self) -> Result<Option<usize>, ()> {
        for event in self.event_pump.poll_iter() {

            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    return Err(());
                },
                Event::KeyDown {keycode: Some(key), ..} => {
                    if let Some(k) = KeypadDriver::keycode_to_keypad(key) {
                        self.key_pressed = true;
                        return Ok(Some(k));
                    }
                },
                Event::KeyUp {keycode: Some(key), ..} => {
                    if let Some(k) = KeypadDriver::keycode_to_keypad(key) {
                        self.key_pressed = false;
                        return Ok(Some(k));
                    }
                },
                _ => (),
            }
        }
        Ok(None)
    }

    // Converts detected keycodes to CHIP-8 keypad keys
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

}