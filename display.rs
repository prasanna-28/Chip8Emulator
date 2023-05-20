use crossterm::{cursor::MoveTo, queue, style::Print};

pub struct Display {
    width: u32,
    height: u32,
}

impl Display {
    pub fn new() -> Display {
        Display {
            width: 64,
            height: 32,
        }
    }

    pub fn draw(&self, gfx: &[u8; 64 * 32]) {
        let mut stdout = std::io::stdout();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                let pixel = gfx[index];
                let character = if pixel == 0 { ' ' } else { '#' };
                queue!(stdout, MoveTo(x as u16, y as u16), Print(character)).unwrap();
            }
        }
    }
}
