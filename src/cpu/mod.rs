use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;

use crate::cartridge::info::Info;
use crate::mem::Mem;
use crate::util::{InstrSize, MemAddr};

use log::{trace, warn};

const CARRY_STATUS_FLAG: u8 = 0b0000_0001;
const ZERO_STATUS_FLAG: u8 = 0b0000_0010;
const IRQ_DISABLED_MODE_FLAG: u8 = 0b0000_0100;
const DECIMAL_MODE_FLAG: u8 = 0b0000_1000;
const PROGRAM_BREAK_INTERRUPT_FLAG: u8 = 0b0001_0000;
const INDEX_8BIT_MODE_FLAG: u8 = 0b0001_0000;
const ACC_8BIT_MODE_FLAG: u8 = 0b0010_0000;
const OVERFLOW_STATUS_FLAG: u8 = 0b0100_0000;
const NEGATIVE_STATUS_FLAG: u8 = 0b1000_0000;

pub struct Cpu {
    mem: Arc<RwLock<Mem>>,
    pc: MemAddr,
    flags: u8,
    emulation: bool,

    acc: u16,
    dp: u16,

    pub stop: bool,
}

impl Cpu {
    pub fn new(mem: Arc<RwLock<Mem>>, cart_info: &Info) -> Cpu {
        Cpu {
            mem,
            pc: cart_info.reset_vector,
            flags: IRQ_DISABLED_MODE_FLAG, // IRQ set, others not set
            emulation: true,
            acc: 0x0000,
            dp: 0x0000,
            stop: false,
        }
    }

    fn next_instr(&mut self) {
        let instr = self.mem.read().unwrap()[self.pc];

        let instr_size = self.handle_instr(instr);
        self.pc = self.pc + instr_size;
    }

    fn handle_instr(&mut self, instr: u8) -> InstrSize {
        match instr {
            0x18 => {
                trace!("CLC");
                self.clear_flag(CARRY_STATUS_FLAG);
                self.trace_flags();
                InstrSize(1)
            }
            0x5B => {
                trace!("TCD");
                self.dp = self.acc;

                self.set_transfer_flags(self.dp, 2);

                InstrSize(1)
            }
            0x78 => {
                trace!("SEI");
                self.set_flag(IRQ_DISABLED_MODE_FLAG);
                self.trace_flags();
                InstrSize(1)
            }
            0x8D => {
                let addr = self.mem_read_addr(2);
                trace!("STA {:?}", addr);
                warn!("TODO Implement STA 0x8D");
                InstrSize(3)
            }
            0x9C => {
                let addr = self.mem_read_addr(2);
                trace!("STZ {:?}", addr);
                warn!("TODO Implement STZ 0x9C");
                InstrSize(3)
            }
            0xA9 => {
                let const_size = if self.m() { 1 } else { 2 };
                let const_value = self.mem_read_value(const_size);
                trace!("LDA {:04X}", const_value);

                if self.m() {
                    // only set the low byte in 8-bit mode
                    self.acc &= 0xFF00; // clear low byte
                    self.acc |= const_value; // set low byte
                } else {
                    self.acc = const_value;
                }

                trace!("ACC {:04X}", self.acc);

                self.set_transfer_flags(const_value, const_size as u16);

                InstrSize(const_size + 1)
            }
            0xC2 => {
                let bits_to_clear = self.mem_read_value(1);
                trace!("REP {:08b}", bits_to_clear);
                self.clear_flag(bits_to_clear as u8);
                self.trace_flags();
                InstrSize(2)
            }
            0xFB => {
                trace!("XCE");
                let new_emu_mode = self.is_flag_set(CARRY_STATUS_FLAG);

                if self.emulation {
                    self.set_flag(CARRY_STATUS_FLAG);
                } else {
                    self.clear_flag(CARRY_STATUS_FLAG);
                }

                self.emulation = new_emu_mode;

                if self.emulation {
                    self.clear_flag(PROGRAM_BREAK_INTERRUPT_FLAG);
                } else {
                    self.set_flag(ACC_8BIT_MODE_FLAG);
                    self.set_flag(INDEX_8BIT_MODE_FLAG);
                }

                self.trace_flags();

                InstrSize(1)
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

        self.is_flag_set(ACC_8BIT_MODE_FLAG)
    }

    fn set_flag(&mut self, flag: u8) {
        self.flags |= flag;
    }

    fn set_transfer_flags(&mut self, transfer_value: u16, transfer_size: u16) {
        if transfer_value & (0x0080 * (1 + transfer_size * 0x10)) != 0 {
            self.set_flag(NEGATIVE_STATUS_FLAG);
        }

        if transfer_value == 0 {
            self.set_flag(ZERO_STATUS_FLAG);
        }

        self.trace_flags();
    }

    fn clear_flag(&mut self, flag: u8) {
        self.flags &= 0xFF - flag;
    }

    fn is_flag_set(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    fn mem_read_value(&self, size: usize) -> u16 {
        self.mem.read().unwrap().read_value(self.pc + 1, size)
    }

    fn mem_read_addr(&self, size: usize) -> MemAddr {
        self.mem.read().unwrap().read_addr(self.pc + 1, size)
    }

    fn trace_flags(&self) {
        trace!("FLAGS {:08b} EMULATION {}", self.flags, self.emulation);
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
