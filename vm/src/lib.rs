mod cpu;
pub mod instructions;
mod memory;
pub mod registers;
mod screen_device;

mod traits {
    use crate::types::*;

    pub trait Read {
        fn get_u8(&self, addr: Addr) -> Byte;
        fn get_u16(&self, addr: Addr) -> Short;
    }

    pub trait Write {
        fn set_u8(&mut self, addr: Addr, val: Byte);
        fn set_u16(&mut self, addr: Addr, val: Short);
    }

    pub trait Device: Read + Write + std::fmt::Debug {}
}

mod types {
    pub type Addr = u16;
    pub type Byte = u8;
    pub type Short = u16;
}

pub mod prelude {
    pub use crate::cpu::Cpu;
    pub use crate::instructions::{
        InstructionArguments,
        InstructionVariant,
    };
    pub use crate::memory::{
        Memory,
        MemoryMapper,
        MemoryRegion,
    };
    pub use crate::registers::{
        Register,
        RegisterVariant,
    };
    pub use crate::traits::*;
    pub use crate::types::*;
    pub use crate::screen_device::*;
}
