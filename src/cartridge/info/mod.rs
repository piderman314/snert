mod game_title;

use game_title::GameTitle;

use crate::cartridge::Cartridge;

pub struct Info<'cartridge> {
    pub game_title: GameTitle<'cartridge>,
}

pub fn from_cartridge(cartridge: &Cartridge) -> Info {
    Info {
        game_title: game_title::from_cartridge(cartridge),
    }
}
