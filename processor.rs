use crate::chip8::Chip8;

pub trait Processor {
    fn process_opcode(&mut self, opcode: u16);
}

impl Processor for Chip8 {
    fn process_opcode(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x1000 => {
                self.pc = opcode & 0x0FFF;
            },
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            },
            0x3000 => {
                let vx = (opcode & 0x0F00) >> 8;
                let nn = (opcode & 0x00FF) as u8;
                if self.v[vx as usize] == nn {
                    self.pc += 2;
                }
            },
            //... Implement other opcodes here
            _ => println!("Unknown opcode: {:X}", opcode),
        }

        self.pc += 2;
    }
}
