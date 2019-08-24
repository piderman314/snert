use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::cartridge::info::Info;
use crate::mem::{Mem, MemAddr};

use log::info;

pub struct Cpu {
    mem: Arc<RwLock<Mem>>,
    pc: MemAddr,

    pub stop: bool,
}

impl Cpu {
    pub fn new(mem: Arc<RwLock<Mem>>, cart_info: &Info) -> Cpu {
        Cpu {
            mem,
            pc: cart_info.reset_vector,
            stop: false,
        }
    }
}

pub fn start(cpu: Arc<RwLock<Cpu>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        {
            let c = cpu.read().unwrap();
            if c.stop {
                break;
            }
        }

        info!("IN THREAD");

        thread::sleep(Duration::from_secs(1));
    })
}
