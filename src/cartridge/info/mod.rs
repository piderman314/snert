mod game_title;

use crate::util::MemAddr;
use game_title::GameTitle;

use crate::cartridge::{CartAddr, Cartridge};

pub struct Info<'cartridge> {
    pub game_title: GameTitle<'cartridge>,
    pub reset_vector: MemAddr,
}

pub fn from_cartridge(cartridge: &Cartridge) -> Info {
    Info {
        game_title: game_title::from_cartridge(cartridge),
        reset_vector: reset_vecor(cartridge),
    }
}

const RESET_VECTOR_OFFSET: usize = 0x7FFC;

fn reset_vecor(cartridge: &Cartridge) -> MemAddr {
    let rv = cartridge[CartAddr(RESET_VECTOR_OFFSET)] as usize
        + ((cartridge[CartAddr(RESET_VECTOR_OFFSET + 1)] as usize) << 8);

    MemAddr(rv)
}
