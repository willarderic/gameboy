use std::ops::Index;

pub enum Region {
    FixedROM = 0x0000,
    SwitchableROM = 0x4000,
    VRAM = 0x8000,
    ExternalRAM = 0xA000,
    WRAMC = 0xC000,
    WRAMD = 0xD000,
    EchoRAM = 0xE000,
    OAM = 0xFE00,
    Unused = 0xFEA0,
    IO = 0xFF00,
    HRAM = 0xFF80,
    IE = 0xFFFF,
}

// The annotation lets the struct be initialized to zero via Memory::default()
pub struct Memory {
    memory: [u8; 0xffff],
}

impl Default for Memory {
    fn default() -> Memory {
        Memory {
            memory: [0; 0xffff],
        }
    }
}

impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, i: u16) -> &Self::Output {
        &self.memory[i as usize]
    }
}

impl Memory {
    // to make changes to the struct, a mutable self reference must be passed
    // in as the first parameter
    pub fn write(&mut self, address: u16, byte: u8) {
        // indexing into an array can only index with `usize` unless
        // the Index trait is implemented for the memory type with u16
        self.memory[address as usize] = byte;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
