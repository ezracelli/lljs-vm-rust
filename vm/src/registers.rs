use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegisterVariant {
    Ip,
    Acc,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    Sp,
    Fp,
}

impl From<Byte> for RegisterVariant {
    fn from(i: Byte) -> Self {
        match i {
            constants::IP => Self::Ip,
            constants::ACC => Self::Acc,
            constants::R1 => Self::R1,
            constants::R2 => Self::R2,
            constants::R3 => Self::R3,
            constants::R4 => Self::R4,
            constants::R5 => Self::R5,
            constants::R6 => Self::R6,
            constants::R7 => Self::R7,
            constants::R8 => Self::R8,
            constants::SP => Self::Sp,
            constants::FP => Self::Fp,
            _ => unimplemented!("unknown register `{:#x?}`", i),
        }
    }
}

impl Into<Byte> for RegisterVariant {
    fn into(self) -> Byte {
        match self {
            Self::Ip => constants::IP,
            Self::Acc => constants::ACC,
            Self::R1 => constants::R1,
            Self::R2 => constants::R2,
            Self::R3 => constants::R3,
            Self::R4 => constants::R4,
            Self::R5 => constants::R5,
            Self::R6 => constants::R6,
            Self::R7 => constants::R7,
            Self::R8 => constants::R8,
            Self::Sp => constants::SP,
            Self::Fp => constants::FP,
        }
    }
}

#[derive(Debug)]
pub struct Register {
    pub memory: Memory,
}

impl Register {
    pub fn new() -> Self {
        Self {
            memory: Memory::with_capacity(2),
        }
    }
}

impl Read for Register {
    fn get_u8(&self, addr: Addr) -> Byte { self.memory.get_u8(addr) }
    fn get_u16(&self, addr: Addr) -> Short { self.memory.get_u16(addr) }
}

impl Write for Register {
    fn set_u8(&mut self, addr: Addr, val: Byte) { self.memory.set_u8(addr, val); }
    fn set_u16(&mut self, addr: Addr, val: Short) { self.memory.set_u16(addr, val); }
}

impl Device for Register {}

pub mod constants {
    use super::*;

    pub const IP: Byte = 0x00;
    pub const ACC: Byte = 0x01;
    pub const R1: Byte = 0x02;
    pub const R2: Byte = 0x03;
    pub const R3: Byte = 0x04;
    pub const R4: Byte = 0x05;
    pub const R5: Byte = 0x06;
    pub const R6: Byte = 0x07;
    pub const R7: Byte = 0x08;
    pub const R8: Byte = 0x09;
    pub const SP: Byte = 0x0A;
    pub const FP: Byte = 0x0B;
}
