use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::util::u8_slice_to_hex;

#[derive(Default)]
pub struct Cartridge {
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn read_cartridge(&mut self, filename: &String) -> io::Result<()> {
        // The ? operator will continue if receiving Ok from the function
        // and return early with "Err" if something went wrong
        let mut f = File::open(filename)?;
        f.read_to_end(&mut self.rom)?;

        Ok(())
    }

    pub fn print_header(&self) {
        println!("------ CARTRIDGE HEADER ------");
        println!(
            "\tEntry bytes = 0x{}",
            u8_slice_to_hex(&self.rom[0x100..0x104])
        );
        println!(
            "\tNintendo Logo = 0x{}",
            u8_slice_to_hex(&self.rom[0x104..0x134])
        );
        println!(
            "\tTitle = {}",
            String::from_utf8_lossy(&self.rom[0x134..0x143])
        );
        println!("\tCGB Flag = {:#02x}", self.rom[0x144]);
        println!(
            "\tLicensee code = 0x{}",
            u8_slice_to_hex(&self.rom[0x144..0x146])
        );
        println!("\tSGB Flag = {:#02x}", self.rom[0x146]);
        println!("\tCartridge Type = {}", get_cartridge_type(self.rom[0x147]));
        println!("\tROM size = {}", get_rom_size(self.rom[0x148]));
        println!("\tRAM size = {}", get_ram_size(self.rom[0x149]));
        println!("\tDestination Code = {:#02x}", self.rom[0x14a]);
        println!("\tOld licensee code = {:#02x}", self.rom[0x14b]);
        println!("\tVersion Number = {:#02x}", self.rom[0x14c]);
        println!("\tHeader checksum = {:#02x}", self.rom[0x14d]);
        println!(
            "\tGlobal checksum = 0x{}",
            u8_slice_to_hex(&self.rom[0x14e..0x150])
        );
        println!("-------- HEADER END ----------");
    }
}

fn get_cartridge_type(t: u8) -> String {
    match t {
        0x00 => String::from("ROM ONLY"),
        0x01 => String::from("MBC1"),
        0x02 => String::from("MBC1+RAM"),
        0x03 => String::from("MBC1+RAM+BATTERY"),
        0x05 => String::from("MBC2"),
        0x06 => String::from("MBC2+BATTERY"),
        0x08 => String::from("ROM+RAM "),
        0x09 => String::from("ROM+RAM+BATTERY "),
        0x0b => String::from("MMM01"),
        0x0c => String::from("MMM01+RAM"),
        0x0d => String::from("MMM01+RAM+BATTERY"),
        0x0f => String::from("MBC3+TIMER+BATTERY"),
        0x10 => String::from("MBC3+TIMER+RAM+BATTERY"),
        0x11 => String::from("MBC3"),
        0x12 => String::from("MBC3+RAM"),
        0x13 => String::from("MBC3+RAM+BATTERY"),
        0x19 => String::from("MBC5"),
        0x1a => String::from("MBC5+RAM"),
        0x1b => String::from("MBC5+RAM+BATTERY"),
        0x1c => String::from("MBC5+RUMBLE"),
        0x1d => String::from("MBC5+RUMBLE+RAM"),
        0x1e => String::from("MBC5+RUMBLE+RAM+BATTERY"),
        0x20 => String::from("MBC6"),
        0x22 => String::from("MBC7+SENSOR+RUMBLE+RAM+BATTERY"),
        0xfc => String::from("POCKET CAMERA"),
        0xfd => String::from("BANDAI TAMA5"),
        0xfe => String::from("HuC3"),
        0xff => String::from("HuC1+RAM+BATTERY"),
        unknown => format!("UNKNOWN CARTRIDGE TYPE: {:#02x}", unknown),
    }
}

fn get_rom_size(value: u8) -> String {
    match value {
        0x00 => String::from("Size: 32 KiB, # of ROM Banks: 2 (no banking)"),
        0x01 => String::from("Size: 64 KiB, # of ROM Banks: 4"),
        0x02 => String::from("Size: 128 KiB, # of ROM Banks: 8"),
        0x03 => String::from("Size: 256 KiB, # of ROM Banks: 16"),
        0x04 => String::from("Size: 512 KiB, # of ROM Banks: 32"),
        0x05 => String::from("Size: 1 MiB, # of ROM Banks: 64"),
        0x06 => String::from("Size: 2 MiB, # of ROM Banks: 128"),
        0x07 => String::from("Size: 4 MiB, # of ROM Banks: 256"),
        0x08 => String::from("Size: 8 MiB, # of ROM Banks: 512"),
        unknown => format!("UNKNOWN ROM SIZE: {:#02x}", unknown),
    }
}

fn get_ram_size(value: u8) -> String {
    match value {
        0x00 => String::from("No RAM"),
        0x01 => String::from("Unused"),
        0x02 => String::from("8 KiB; 1 bank"),
        0x03 => String::from("32 KiB; 4 banks of 8 KiB each"),
        0x04 => String::from("128 KiB; 16 banks of 8 KiB each"),
        0x05 => String::from("64 kIb; 8 banks of 8 KiB each"),
        unknown => format!("UNKNOWN RAM SIZE: {:#02x}", unknown),
    }
}
