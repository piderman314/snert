pub mod info;

use std::fs::File;
use std::io::prelude::Read;
use std::ops::Index;

pub struct CartAddr(pub usize);
pub struct CartAddrRange(pub CartAddr, pub CartAddr);

pub struct Cartridge {
    raw_data: Vec<u8>,
}

impl Index<CartAddr> for Cartridge {
    type Output = u8;

    fn index(&self, addr: CartAddr) -> &u8 {
        &self.raw_data[addr.0]
    }
}

impl Index<CartAddrRange> for Cartridge {
    type Output = [u8];

    fn index(&self, addr: CartAddrRange) -> &[u8] {
        &self.raw_data[(addr.0).0..(addr.1).0]
    }
}

pub fn from_rom_file(rom_filename: &str) -> Cartridge {
    let mut f = File::open(rom_filename).expect("File not found");
    let mut raw_data = Vec::new();
    f.read_to_end(&mut raw_data).expect("Failed to read file");

    Cartridge { raw_data }
}
