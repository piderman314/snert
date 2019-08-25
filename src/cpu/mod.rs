use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;

use crate::cartridge::info::Info;
use crate::mem::Mem;
use crate::util::{InstrSize, MemAddr};

use log::{trace, warn};

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

    fn next_instr(&mut self) {
        let instr = self.mem.read().unwrap()[self.pc];

        let instr_size = self.handle_instr(&mut self.mem.write().unwrap(), instr);
        self.pc = self.pc + instr_size;
    }

    fn handle_instr(&self, mem: &mut Mem, instr: u8) -> InstrSize {
        match instr {
            0x78 => {
                trace!("SEI");
                warn!("TODO Implement 0x78 SEI");
                InstrSize(1)
            }
            0x9C => {
                let addr = mem.read_addr(self.pc + 1, 2);
                trace!("STZ {:?}", addr);
                warn!("TODO Implement 0x9C STZ");
                InstrSize(3)
            }
            _ => {
                panic!("Unknown instruction {:02X}", instr);
            }
        }
    }
}

pub fn start(cpu: Arc<RwLock<Cpu>>) -> JoinHandle<()> {
    let builder = thread::Builder::new().name("cpu".into());

    builder
        .spawn(move || loop {
            let mut cpu_w = cpu.write().unwrap();

            if cpu_w.stop {
                break;
            }

            cpu_w.next_instr();
        })
        .unwrap()
}
