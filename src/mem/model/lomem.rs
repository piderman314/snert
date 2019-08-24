use super::{MemType, Model, RelativeAddr};
use crate::mem::MemAddr;

pub struct LoMem {}

impl Model for LoMem {
    fn map_raw_addr(&self, raw_addr: MemAddr) -> RelativeAddr {
        let bank = raw_addr.0 / 0xFFFF;
        let addr = raw_addr.0 & 0xFFFF;
        match bank {
            0x00..=0x3F => {
                self.map_lower_banks(bank, addr)
            },
            _ => {
                panic!("Memory Address out of range for LoMem: ${:02X}:{:04X}", bank, addr);
            }
        }
    }
}

impl LoMem {
    pub fn new() -> LoMem {
        LoMem {}
    }

    fn map_lower_banks(&self, bank: usize, addr: usize) -> RelativeAddr {
        match addr {
            0x8000..=0xFFFF => {
                RelativeAddr(MemType::Cartridge, bank * 0x10000 + addr - 0x8000)
            },
            _ => {
                panic!("Memory Address out of range for LoMem: ${:02X}:{:04X}", bank, addr);
            }
        }
    }
}
