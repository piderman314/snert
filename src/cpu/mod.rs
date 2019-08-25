use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;

use crate::cartridge::info::Info;
use crate::mem::Mem;
use crate::util::{InstrSize, MemAddr};

use log::{trace, warn};

const CARRY_STATUS_FLAG: u8 = 0b0000_0001;
const ZERO__STATUS_FLAG: u8 = 0b0000_0010;
const IRQ_DISABLED_MODE_FLAG: u8 = 0b0000_0100;
const DECIMAL_MODE_FLAG: u8 = 0b0000_1000;
const INDEX_8BIT_MODE_FLAG: u8 = 0b0001_0000;
const ACC_8BIT_MODE_FLAG: u8 = 0b0010_0000;
const OVERFLOW_STATUS_FLAG: u8 = 0b0100_0000;
const NEGATIVE_STATUS_FLAG: u8 = 0b1000_0000;

pub struct Cpu {
    mem: Arc<RwLock<Mem>>,
    pc: MemAddr,
    flags: u8,
    emulation: bool,

    pub stop: bool,
}

impl Cpu {
    pub fn new(mem: Arc<RwLock<Mem>>, cart_info: &Info) -> Cpu {
        Cpu {
            mem,
            pc: cart_info.reset_vector,
            flags: IRQ_DISABLED_MODE_FLAG, // IRQ set, others not set
            emulation: true,
            stop: false,
        }
    }

    fn next_instr(&mut self) {
        let instr = self.mem.read().unwrap()[self.pc];

        let instr_size = self.handle_instr(instr);
        self.pc = self.pc + instr_size;
    }

    fn handle_instr(&mut self, instr: u8) -> InstrSize {
        let mut mem = self.mem.write().unwrap();
        match instr {
            0x78 => {
                trace!("SEI");
                self.flags &= IRQ_DISABLED_MODE_FLAG;
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

    fn m(&self) -> bool {
        if self.emulation {
            return true;
        }

        self.flags & ACC_8BIT_MODE_FLAG != 0
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
