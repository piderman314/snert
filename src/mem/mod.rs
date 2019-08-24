pub mod model;

use std::ops::Index;
use std::sync::Arc;

use crate::cartridge::{CartAddr, Cartridge};
use model::{MemType, Model, RelativeAddr};

#[derive(Copy, Clone, Debug)]
pub struct MemAddr(pub usize);

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
}
