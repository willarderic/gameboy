use std::env;

mod bus;
mod cartridge;
mod util;

use bus::Memory;
use cartridge::Cartridge;
use util::u8_slice_to_hex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let memory = Memory::default();
    let mut cartridge = Cartridge::default();
    for i in 0..10 {
        println!("memory[{}] = {}", i, memory.read(i));
    }

    cartridge
        .read_cartridge(filename)
        .expect("failed to open cartridge");

    cartridge.print_header();
}
