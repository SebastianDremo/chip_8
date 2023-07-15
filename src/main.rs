#![allow(dead_code, unused_variables)]

use chip8::Chip8;

mod chip8;
mod font;
mod memory;

fn main() {
    let processor: Chip8 = Chip8::new();
    let rom = processor.run_rom(String::from("../roms/test_opcode.ch8"));
}
