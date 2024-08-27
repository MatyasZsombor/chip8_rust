use std::fs;

pub struct Chip8 {
    screen: [bool; 64 * 32],
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
            screen: [true; 64 * 32],
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
        println!("{:#02x}", self.memory[0x201])
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

    pub fn tick(&mut self) {}
}
