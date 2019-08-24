pub mod lomem;

use crate::mem::MemAddr;

pub enum MemType {
    Cartridge,
}

pub struct RelativeAddr(pub MemType, pub usize);

pub trait Model {
    fn map_raw_addr(&self, raw_addr: MemAddr) -> RelativeAddr;
}
