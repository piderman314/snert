pub mod model;

use std::ops::Index;
use std::sync::Arc;

use crate::cartridge::{CartAddr, Cartridge};
use crate::util::MemAddr;
use model::{MemType, Model, RelativeAddr};

pub struct Mem {
    pub model: Box<Model + Send + Sync>,
    pub cartridge: Arc<Cartridge>,
}

impl Index<MemAddr> for Mem {
    type Output = u8;

    fn index(&self, addr: MemAddr) -> &u8 {
        self.access(addr)
    }
}

impl Mem {
    fn access(&self, addr: MemAddr) -> &u8 {
        let RelativeAddr(mem_type, cart_addr) = self.model.map_raw_addr(addr);

        match mem_type {
            MemType::Cartridge => &self.cartridge[CartAddr(cart_addr)],
        }
    }

    pub fn read_value(&self, addr_addr: MemAddr, size: usize) -> u16 {
        let mut value: u16 = 0;

        for i in 0..size {
            value += u16::from(self[addr_addr + i]) << (8 * i);
        }

        value
    }

    pub fn read_addr(&self, addr_addr: MemAddr, size: usize) -> MemAddr {
        let mut addr: usize = 0;

        for i in 0..size {
            addr += ((self[addr_addr + i]) as usize) << (8 * i);
        }

        MemAddr(addr)
    }
}
