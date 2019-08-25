use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct MemAddr(pub usize);

impl fmt::Debug for MemAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:02X}:{:04X}", self.0 / 0xFFFF, self.0 & 0xFFFF)
    }
}

pub struct InstrSize(pub usize);

impl Add<InstrSize> for MemAddr {
    type Output = MemAddr;

    fn add(self, other: InstrSize) -> MemAddr {
        MemAddr(self.0 + other.0)
    }
}

impl Add<usize> for MemAddr {
    type Output = MemAddr;

    fn add(self, other: usize) -> MemAddr {
        MemAddr(self.0 + other)
    }
}
