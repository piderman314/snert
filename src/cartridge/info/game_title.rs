use std::str;

use crate::cartridge::Cartridge;
use crate::cartridge::{CartAddr, CartAddrRange};

const GAME_TITLE_OFFSET: usize = 0x7FC0;

#[derive(Debug)]
pub struct GameTitle<'cartridge> {
    title: &'cartridge str,
}

pub fn from_cartridge(cartridge: &Cartridge) -> GameTitle {
    let title = &cartridge[CartAddrRange(
        CartAddr(GAME_TITLE_OFFSET),
        CartAddr(GAME_TITLE_OFFSET + 21),
    )];
    let title = str::from_utf8(&title).unwrap();

    GameTitle { title }
}
