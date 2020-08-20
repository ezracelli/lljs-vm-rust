use vm::prelude::*;
use vm::instructions::constants::*;
use vm::registers::constants::*;
// use std::rc::Rc;

fn main() {
    let mut memory = Memory::with_capacity(0x10000);
    let mut addr: Addr = 0x0000;
    let mut mm_addr: Addr = 0x3000;

    let mut write_char = |command: u8, char: u8| {
        memory.set_u8(addr, MOV_LIT_REG);
        addr += 1;
        memory.set_u8(addr, command);
        addr += 1;
        memory.set_u8(addr, char);
        addr += 1;
        memory.set_u8(addr, R1);
        addr += 1;

        let mm_addr_hi = (mm_addr / 0x100) as u8;
        let mm_addr_lo = (mm_addr % 0x100) as u8;

        memory.set_u8(addr, MOV_REG_MEM);
        addr += 1;
        memory.set_u8(addr, R1);
        addr += 1;
        memory.set_u8(addr, mm_addr_hi);
        addr += 1;
        memory.set_u8(addr, mm_addr_lo);
        addr += 1;

        mm_addr += 1;
    };

    write_char(0xff, 0x00);
    write_char(0x01, 0x00);

    write_char(0x00, b'H');
    write_char(0x00, b'e');
    write_char(0x00, b'l');
    write_char(0x00, b'l');
    write_char(0x00, b'o');

    write_char(0x02, 0x00);

    write_char(0x00, b' ');
    write_char(0x00, b'W');
    write_char(0x00, b'o');
    write_char(0x00, b'r');
    write_char(0x00, b'l');
    write_char(0x00, b'd');
    write_char(0x00, b'!');
    write_char(0x00, b'\n');

    write_char(0x02, 0x00);

    memory.set_u8(addr, HLT);

    let screen = ScreenDevice::new();

    let mut mm = MemoryMapper::new();

    mm.add_region(
        MemoryRegion::builder()
            .range(0x3000..=0x30ff)
            .device(Box::new(screen))
            .finalize()
            .unwrap(),
    );

    mm.add_region(
        MemoryRegion::builder()
            .range(memory.get_range())
            .device(Box::new(memory))
            .finalize()
            .unwrap(),
    );

    let mut cpu = Cpu::from(mm);
    cpu.run();
}
