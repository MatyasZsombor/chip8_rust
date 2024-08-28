use crate::consts;
use std::fs;

pub struct Chip8 {
    screen: [bool; consts::WIDTH * consts::HEIGHT],
    memory: [u8; 4096],
    pc: u16,
    index_register: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            memory: [0; 4096],
            screen: [false; 64 * 32],
            pc: 0,
            index_register: 0,

            stack: vec![],

            delay_timer: 255,
            sound_timer: 255,
            registers: [0; 16],
        };

        //LOADS FONT STARTING AT 0x50
        chip8.load_into_memory(
            vec![
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],
            0x50,
        );
        chip8
    }

    pub fn load_rom(&mut self, file_name: &String) {
        self.load_into_memory(
            fs::read(file_name).unwrap_or_else(|_| panic!("Couldn't find file: {}", file_name)),
            0x200,
        );
        self.pc = 0x200;
    }

    pub fn get_screen(&self) -> &[bool; 64 * 32] {
        &self.screen
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn load_into_memory(&mut self, data: Vec<u8>, start_address: usize) {
        self.memory[start_address..(data.len() + start_address)].copy_from_slice(&data[..]);
    }

    fn read_u8(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn read_u16(&self, address: u16) -> u16 {
        ((self.read_u8(address) as u16) << 8) | (self.read_u8(address + 1) as u16)
    }

    pub fn tick(&mut self) {
        let instruction = self.read_u16(self.pc);
        let opcode = (instruction & 0xF000) >> 12;
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        let n = (instruction & 0x000F) as u8;

        let second_byte = (instruction & 0xFF) as u8;
        let memory_address = instruction & 0xFFF;

        self.pc += 2;

        match opcode {
            0x0 => self.clear_scene(),
            0x1 => self.pc = memory_address,
            0x6 => self.registers[x] = second_byte,
            0x7 => self.registers[x] = self.registers[x].wrapping_add(second_byte),
            0xA => self.index_register = memory_address,
            0xD => self.display(x, y, n as usize),
            _ => panic!("Unknown opcode {:x}", opcode),
        }
    }

    fn display(&mut self, x: usize, y: usize, n: usize) {
        let x = self.registers[x] as usize % consts::WIDTH;
        let y = self.registers[y] as usize % consts::HEIGHT;
        self.registers[0xf] = 0;

        for y_pos in 0..n {
            if y + y_pos >= consts::HEIGHT {
                break;
            }

            let sprite_row = self.read_u8(self.index_register + y_pos as u16);

            for x_pos in 0..8 {
                if x + x_pos >= consts::WIDTH {
                    break;
                }

                let idx = (y + y_pos) * consts::WIDTH + x + x_pos;
                if (sprite_row & (0b1000_0000 >> x_pos)) != 0 {
                    self.screen[idx] ^= true;
                }
            }
        }
    }

    fn clear_scene(&mut self) {
        for i in 0..self.screen.len() {
            self.screen[i] = false;
        }
    }
}
