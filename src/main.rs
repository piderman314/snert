mod cartridge;
mod cpu;
mod mem;
mod util;

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

use std::sync::{Arc, RwLock};

fn main() {
    setup_logging();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Need the ROM filename as argument");
        process::exit(1);
    }

    let rom_file = &args[1];
    info!("Using rom file {}", rom_file);

    let cartridge = Arc::new(cartridge::from_rom_file(rom_file));
    let info = info::from_cartridge(&cartridge);

    info!("GAME TITLE {:?}", info.game_title);

    let memory = Arc::new(RwLock::new(mem::Mem {
        model: Box::new(mem::model::lomem::LoMem::new()),
        cartridge: cartridge.clone(),
    }));

    let c = Arc::new(RwLock::new(cpu::Cpu::new(memory.clone(), &info)));

    let cpu_handle = cpu::start(c.clone());

    cpu_handle.join().unwrap();
}

fn setup_logging() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%F %T%.3f)} {l} {M} \\{{T}\\} {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();
    log4rs::init_config(config).unwrap();
}
