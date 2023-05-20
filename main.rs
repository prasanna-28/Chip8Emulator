mod chip8;
mod processor;
mod display;
mod keyboard;

use chip8::Chip8;
use processor::Processor;
use display::Display;
use keyboard::Keyboard;
use std::time::{Instant, Duration};
use std::thread::sleep;

fn main() {
    let mut chip8 = Chip8::new();
    let mut display = Display::new();
    let mut keyboard = Keyboard::new();

    chip8.load_program(&[0x13, 0xC5, 0x23, 0x07]); // Dummy program

    let sixty_hz_duration = Duration::from_millis(1000 / 60);
    
    loop {
        let cycle_start = Instant::now();
        
        let opcode: u16 = (chip8.memory[chip8.pc as usize] as u16) << 8 | (chip8.memory[(chip8.pc+1) as usize] as u16);

        chip8.process_opcode(opcode);

        // Update the display
        display.draw(&chip8.gfx);

        // Check for input
        chip8.key = keyboard.get_keys();

        if chip8.delay_timer > 0 {
            chip8.delay_timer -= 1;
        }

        if chip8.sound_timer > 0 {
            if chip8.sound_timer == 1 {
                println!("BEEP!\n");
            }
            chip8.sound_timer -= 1;
        }

        let cycle_end = Instant::now();
        let cycle_duration = cycle_end.duration_since(cycle_start);
        
        if cycle_duration < sixty_hz_duration {
            sleep(sixty_hz_duration - cycle_duration);
        }
    }
}
