mod cartridge;

use std::env;
use std::process;

use log::info;
use log::LevelFilter;

use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;

use cartridge::info;

fn main() {
    setup_logging();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Need the ROM filename as argument");
        process::exit(1);
    }

    let rom_file = &args[1];
    info!("Using rom file {}", rom_file);

    let cartridge = cartridge::from_rom_file(rom_file);
    let info = info::from_cartridge(&cartridge);

    info!("GAME TITLE {:?}", info.game_title);
}

fn setup_logging() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%F %T%.3f)} {l} {M} \\{{T}\\} {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(config).unwrap();
}
