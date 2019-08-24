mod cartridge;
mod cpu;
mod mem;

use std::env;
use std::process;
use std::thread;
use std::time::Duration;

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

    info!("ACCESS {:02X}", memory.read().unwrap()[mem::MemAddr(0x00_8000)]);

    let c = Arc::new(RwLock::new(cpu::Cpu::new(memory.clone(), &info)));

    let cpu_handle = cpu::start(c.clone());

    thread::sleep(Duration::from_secs(5));

    c.write().unwrap().stop = true;

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
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(config).unwrap();
}
