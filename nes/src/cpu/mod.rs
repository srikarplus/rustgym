mod addressing;
mod flags;
mod instruction;
mod opcode;

use crate::base::*;
use crate::bus::*;
use crate::iomap::IoMap;
use colored::*;
use flags::Flags;
use opcode::*;

pub const CYCLE_MAX: u32 = !0;
pub const RAM_SIZE: usize = SIZE_2K;

pub enum IrqLine {
    IRQ_EXT = 0x01,
    IRQ_FRAME = 0x40,
    IRQ_DMC = 0x80,
}

pub enum Level {
    LEVEL_LOW = 1,
    LEVEL_HIGH = 9,
    LEVEL_HIGHEST = 10,
}

// fn reset(&mut self);
// fn set_ram_power_state(&mut self, power_state: u32);
// fn boot(&mut self, hard: bool);
// fn execute_frame(&mut self, sound: &impl Sound);
// fn end_frame(&mut self);
// fn power_off(&mut self);
// fn do_mni(&mut self, cycle: Cycle);
// fn do_irq(&mut self, irq_line: IrqLine, cycle: Cycle);
// fn peek(&self, address: usize) -> u32;
// fn poke(&self, address: usize, data: u32);
// fn is_odd_cycle(&self) -> bool;
// fn is_write_cycle(&self, cycle: Cycle) -> bool;
// fn get_clock_base(&self) -> Cycle;
// fn get_clock_divider(&self) -> u32;
// fn get_time(cycle: Cycle) -> u32;
// fn get_fps(&self) -> u32;
// fn set_model(&mut self, model: CpuModel);
// fn add_hook(hook: &impl Hook);
// fn remove_hook(hook: &impl Hook);
// fn push8(&mut self, data: u8);
// fn push16(&mut self, data: u16);
// fn pull8(&mut self);
// fn fetch8(&mut self) -> u8;
// fn fetch16(&mut self) -> u16;
// fn immediate_read(&mut self) -> u8;
// fn zero_page_read(&mut self) -> u8;

#[derive(Debug)]
pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    s: u8,
    flags: Flags,
    pc: u16,
    bus: Bus,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            s: 0xFD,
            pc: 0,
            flags: Flags::new(),
            bus: Bus::new(),
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.s = 0;
        self.pc = 0xFD;
        self.flags.reset();
        self.bus.reset();
    }

    pub fn fetch8(&mut self) -> u8 {
        let byte = self.peek8(self.pc);
        self.pc += 1;
        byte
    }
    pub fn fetch16(&mut self) -> u16 {
        let byte = self.peek16(self.pc);
        self.pc += 2;
        byte
    }
    pub fn push8(&mut self, byte: u8) {
        let addr = 0x100 + self.s as u16;
        self.poke8(addr, byte);
        self.s -= 1;
    }
    pub fn push16(&mut self, word: u16) {
        let bytes = word.to_le_bytes();
        self.push8(bytes[1]);
        self.push8(bytes[0]);
    }
    pub fn pop8(&mut self) -> u8 {
        self.s += 1;
        let addr = 0x100 + self.s as u16;
        self.peek8(addr)
    }
    pub fn pop16(&mut self) -> u16 {
        let bytes = [self.pop8(), self.pop8()];
        u16::from_le_bytes(bytes)
    }
    pub fn exec(&mut self) {
        let opcode = Opcode(self.fetch8());
        opcode.exec(self);
    }
}

impl IoMap for Cpu {
    fn peek8(&self, address: u16) -> u8 {
        self.bus.peek8(address)
    }
    fn poke8(&mut self, address: u16, byte: u8) {
        println!("{} ${:04X} #{:02X}", "poke".red(), address, byte);
        self.bus.poke8(address, byte);
    }
}

#[test]
fn test() {
    let nestest = include_bytes!("../../test/nestest.nes");
    let testlog = include_str!("../../test/nestest.log");
    let mut cpu = Cpu::new();
    cpu.bus.insert_cartridge(nestest);
    cpu.pc = 0xC000;
    for line in testlog.split('\n').take(8980) {
        println!("{}", line);
        let pc = &line[0..4];
        let name = &line[16..16 + 3];
        let a = &line[48..48 + 4];
        let x = &line[53..53 + 4];
        let y = &line[58..58 + 4];
        let p = &line[63..63 + 4];
        let s = &line[68..68 + 5];
        assert_eq!(pc, format!("{:04X}", cpu.pc));
        assert_eq!(name, Opcode(cpu.peek8(cpu.pc)).name());
        assert_eq!(a, format!("A:{:02X}", cpu.a));
        assert_eq!(x, format!("X:{:02X}", cpu.x));
        assert_eq!(y, format!("Y:{:02X}", cpu.y));
        assert_eq!(p, format!("P:{:02X}", cpu.flags.pack()));
        assert_eq!(s, format!("SP:{:02X}", cpu.s));
        cpu.exec();
    }
}
