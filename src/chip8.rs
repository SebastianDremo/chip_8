use std::{fs::File, io::Read};

use crate::font::DEFAULT_FONT;

const RAM_SIZE: usize = 4096; // https://en.wikipedia.org/wiki/CHIP-8#Memory
const ROM_SIZE: usize = 3584;

pub struct Chip8 {
    v: [u8; 16],            // registers
    memory: [u8; RAM_SIZE], // ram
    i: u16,                 // index register
    pc: u16,                // program counter
    sp: u16,                // stack pointer
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    key: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut ram = [0x0; RAM_SIZE];
        for i in 0..DEFAULT_FONT.len() {
            ram[i] = DEFAULT_FONT[i];
        }

        Chip8 {
            v: [0; 16],
            memory: ram,
            i: 0,
            pc: 0x200, // Program's memory start at 0x200, all before it is interpreter memory
            delay_timer: 0x0,
            sound_timer: 0x0,
            stack: [0x0; 16],
            sp: 0x0,
            key: [0x0; 16],
        }
    }

    pub fn run_rom(&mut self, path: String) {
        let (bytes, length) = load_rom(path);
        initialize_memory(self, bytes, length);
    }
}

// Load data from chip-8 ROM at given path.
// Returns data in form of byte array and it's size
fn load_rom(path: String) -> ([u8; ROM_SIZE], usize) {
    let mut f = File::open(&path).expect("Rom not found");
    let mut bytes = [0u8; 3584];

    let bytes_read = f.read(&mut bytes).unwrap();

    (bytes, bytes_read)
}

fn initialize_memory(chip: &mut Chip8, bytes: [u8; ROM_SIZE], length: usize) {
    let addres = 0x200; // Start of program's accessable memory
    for i in 0..length {
        if addres + i > RAM_SIZE {
            panic!("ROM bytes exceded memory size");
        }
        chip.memory[addres + i] = bytes[i];
    }
}
