mod bus;

use bus::Memory;

fn main() {
    let memory = Memory::default();
    for i in 0..100 {
        println!("memory[{}] = {}", i, memory.read(i));
    }
}
