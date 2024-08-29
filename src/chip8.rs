use crate::consts;
use rand::{self, thread_rng, Rng};
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
    keyboard_state: Option<u8>,
    pub wait_int: u8,
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

            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            keyboard_state: None,
            wait_int: 0,
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

    fn wait_for_int(&mut self) -> bool {
        match self.wait_int {
            0 => {
                self.wait_int = 1;
                self.pc -= 2;
                true
            }
            1 => {
                self.pc -= 2;
                true
            }
            _ => {
                self.wait_int = 0;
                false
            }
        }
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

    pub fn set_keyboard(&mut self, state: u8, down: bool) {
        if down {
            self.keyboard_state = None;
        } else {
            self.keyboard_state = Some(state);
        }
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

    fn write_u8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, (value & 0x00FF) as u8);
        self.write_u8(address + 1, (value & 0xFF) as u8)
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

        match (opcode, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => self.clear_scene(),
            (0x0, 0x0, 0xE, 0xE) => self.pc = self.stack.pop().unwrap(),
            (0x1, _, _, _) => self.pc = memory_address,
            (0x2, _, _, _) => {
                self.stack.push(self.pc);
                self.pc = memory_address;
            }
            (0x3, _, _, _) => {
                if self.registers[x] == second_byte {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                if self.registers[x] != second_byte {
                    self.pc += 2;
                }
            }
            (0x5, _, _, 0x0) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => self.registers[x] = second_byte,
            (0x7, _, _, _) => {
                (self.registers[x], _) = self.registers[x].overflowing_add(second_byte)
            }
            (0x8, _, _, 0x0) => self.registers[x] = self.registers[y],
            (0x8, _, _, 0x1) => {
                self.registers[x] |= self.registers[y];
                self.registers[0xF] = 0;
            }
            (0x8, _, _, 0x2) => {
                self.registers[x] &= self.registers[y];
                self.registers[0xF] = 0;
            }
            (0x8, _, _, 0x3) => {
                self.registers[x] ^= self.registers[y];
                self.registers[0xF] = 0;
            }
            (0x8, _, _, 0x4) => {
                let res = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = res.0;
                self.registers[0xF] = if res.1 { 1 } else { 0 };
            }
            (0x8, _, _, 0x5) => {
                let res = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = res.0;
                self.registers[0xF] = if res.1 { 0 } else { 1 };
            }
            (0x8, _, _, 0x6) => {
                self.registers[x] = self.registers[y];
                let r = self.registers[x] & 1;
                self.registers[x] >>= 1;
                self.registers[0xF] = r;
            }
            (0x8, _, _, 0x7) => {
                let res = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = res.0;
                self.registers[0xF] = if res.1 { 0 } else { 1 };
            }
            (0x8, _, _, 0xE) => {
                self.registers[x] = self.registers[y];
                let r = (self.registers[x] & 128) >> 7;
                self.registers[x] = self.registers[x].wrapping_shl(1);
                self.registers[0xF] = r;
            }
            (0x9, _, _, 0x0) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => self.index_register = memory_address,
            (0xB, _, _, _) => self.pc = memory_address + self.registers[0] as u16,
            (0xC, _, _, _) => self.registers[x] = thread_rng().gen::<u8>() & second_byte,
            (0xD, _, _, _) => self.display(x, y, n as usize),
            (0xE, _, 0x9, 0xE) => {
                if self.keyboard_state == Some(self.registers[x]) {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 0x1) => {
                if self.keyboard_state != Some(self.registers[x]) {
                    self.pc += 2;
                }
            }
            (0xF, _, 0x0, 0x7) => {
                self.registers[x] = self.delay_timer;
            }
            (0xF, _, 0x1, 0x5) => {
                self.delay_timer = self.registers[x];
            }
            (0xF, _, 0x1, 0x8) => {
                self.sound_timer = self.registers[x];
            }
            (0xF, _, 1, 0xE) => {
                let res = self.registers[x] as u16 + self.index_register;
                self.registers[0xF] = if res > 0x1000 { 1 } else { 0 };
                self.index_register = res & 0x0FFF;
            }
            (0xF, _, 0x0, 0xA) => {
                if let Some(k) = self.keyboard_state {
                    self.registers[x] = k;
                } else {
                    self.pc -= 2;
                }
            }
            (0xF, _, 0x2, 0x9) => {
                let c = self.registers[x] & 0xF;
                self.index_register = 0x50 + c as u16 * 5;
            }
            (0xF, _, 0x3, 0x3) => {
                let mut num = self.registers[x];
                for i in (0..3).rev() {
                    self.write_u8(self.index_register + i, num % 10);
                    num /= 10;
                }
            }
            (0xF, _, 0x5, 0x5) => {
                for i in 0..=x {
                    self.write_u8(self.index_register + i as u16, self.registers[i]);
                }
                self.index_register += x as u16 + 1;
            }
            (0xF, _, 0x6, 0x5) => {
                for i in 0..=x {
                    self.registers[i] = self.read_u8(self.index_register + i as u16);
                }
                self.index_register += x as u16 + 1;
            }
            _ => println!("Unknown instruction {:x}", instruction),
        }
    }

    fn display(&mut self, x: usize, y: usize, n: usize) {
        if self.wait_for_int() {
            return;
        }

        let x = self.registers[x] as usize % consts::WIDTH;
        let y = self.registers[y] as usize % consts::HEIGHT;
        let mut flipped = false;
        self.registers[0xf] = 0;

        for y_pos in 0..n {
            if y + y_pos >= consts::HEIGHT {
                break;
            }

            let sprite_row = self.read_u8(self.index_register + y_pos as u16);

            for x_pos in 0..8 {
                if x_pos + x >= consts::WIDTH {
                    break;
                }

                if (sprite_row & (0b1000_0000 >> x_pos)) != 0 {
                    let idx = (y + y_pos) * consts::WIDTH + x + x_pos;

                    flipped |= self.screen[idx];
                    self.screen[idx] ^= true;
                }
            }
        }
        self.registers[0xF] = if flipped { 1 } else { 0 };
    }

    fn clear_scene(&mut self) {
        for i in 0..self.screen.len() {
            self.screen[i] = false;
        }
    }
}
