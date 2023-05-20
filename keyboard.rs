use crossterm::event::{self, Event, KeyCode};

pub struct Keyboard {
    // Placeholder for a real keyboard implementation
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {}
    }

    pub fn get_keys(&self) -> [u8; 16] {
        let mut keys = [0u8; 16];

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                match event.code {
                    KeyCode::Char('1') => keys[0x1] = 1,
                    KeyCode::Char('2') => keys[0x2] = 1,
                    KeyCode::Char('3') => keys[0x3] = 1,
                    KeyCode::Char('4') => keys[0xC] = 1,
                    KeyCode::Char('q') => keys[0x4] = 1,
                    KeyCode::Char('w') => keys[0x5] = 1,
                    KeyCode::Char('e') => keys[0x6] = 1,
                    KeyCode::Char('r') => keys[0xD] = 1,
                    KeyCode::Char('a') => keys[0x7] = 1,
                    KeyCode::Char('s') => keys[0x8] = 1,
                    KeyCode::Char('d') => keys[0x9] = 1,
                    KeyCode::Char('f') => keys[0xE] = 1,
                    KeyCode::Char('z') => keys[0xA] = 1,
                    KeyCode::Char('x') => keys[0x0] = 1,
                    KeyCode::Char('c') => keys[0xB] = 1,
                    KeyCode::Char('v') => keys[0xF] = 1,
                    _ => {}
                }
            }
        }

        keys
    }
}
