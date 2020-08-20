use crate::prelude::*;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct Memory(pub Vec<Byte>);

impl Memory {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut inner = Vec::with_capacity(capacity);
        while inner.len() < inner.capacity() {
            inner.push(0);
        }

        Self(inner)
    }

    pub fn get_range(&self) -> RangeInclusive<usize> {
        0x0000..=self.0.capacity()
    }

    pub fn set_bytes<'a>(&mut self, bytes: &'a [Byte]) {
        for (i, byte) in bytes.iter().enumerate() {
            *self.0.get_mut(i).unwrap() = *byte;
        }
    }
}

impl Read for Memory {
    fn get_u8(&self, addr: Addr) -> Byte {
        let addr = addr as usize;
        let entry = self.0.get(addr)
            .expect(&format!("address {:#x?} out of bounds", addr));

        *entry
    }

    fn get_u16(&self, addr: Addr) -> Short {
        let addr = addr as usize;

        let entry_upper = self.0.get(addr)
            .expect(&format!("address {:#x?} out of bounds", addr));
        let entry_lower = self.0.get(addr + 1)
            .expect(&format!("address {:#x?} out of bounds", addr));

        (*entry_upper as Short).checked_shl(0b1000).expect("cannot left shift") + *entry_lower as Short
    }
}

impl Write for Memory {
    fn set_u8(&mut self, addr: Addr, val: Byte) {
        let addr = addr as usize;
        let entry =  self.0.get_mut(addr)
            .expect(&format!("address {:#x?} out of bounds", addr));

        *entry = val;
    }

    fn set_u16(&mut self, addr: Addr, val: Short) {
        let addr = addr as usize;

        {
            let entry_upper = self.0.get_mut(addr)
                .expect(&format!("address {:#x?} out of bounds", addr));

            *entry_upper = val.checked_shr(0b1000).expect("cannot shift right") as Byte;
        }

        {
            let entry_lower = self.0.get_mut(addr + 1)
                .expect(&format!("address {:#x?} out of bounds", addr));

            *entry_lower = val as Byte;
        }
    }
}

impl Device for Memory {}

#[derive(Debug)]
pub struct MemoryRegion {
    pub device: Box<dyn Device>,
    range: RangeInclusive<usize>,
    should_remap: bool,
}

impl MemoryRegion {
    pub fn builder() -> MemoryRegionBuilder {
        MemoryRegionBuilder::default()
    }
}

impl Read for MemoryRegion {
    fn get_u8(&self, addr: Addr) -> Byte { self.device.get_u8(addr) }
    fn get_u16(&self, addr: Addr) -> Short { self.device.get_u16(addr) }
}

impl Write for MemoryRegion {
    fn set_u8(&mut self, addr: Addr, val: Byte) { self.device.set_u8(addr, val); }
    fn set_u16(&mut self, addr: Addr, val: Short) { self.device.set_u16(addr, val); }
}

impl Device for MemoryRegion {}

#[derive(Debug)]
pub struct MemoryRegionBuilderError(String);

impl std::fmt::Display for MemoryRegionBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub struct MemoryRegionBuilder {
    device: Option<Box<dyn Device>>,
    range: Option<RangeInclusive<usize>>,
    should_remap: bool,
}

impl MemoryRegionBuilder {
    pub fn device(mut self, device: Box<dyn Device>) -> Self {
        self.device = Some(device);
        self
    }

    pub fn range(mut self, range: RangeInclusive<usize>) -> Self {
        self.range = Some(range);
        self
    }

    pub fn should_remap(mut self, should_remap: bool) -> Self {
        self.should_remap = should_remap;
        self
    }

    pub fn finalize(self) -> Result<MemoryRegion, MemoryRegionBuilderError> {
        let device = match self.device {
            Some(device) => device,
            None => return Err(MemoryRegionBuilderError("missing field `device`".into())),
        };

        let range = match self.range {
            Some(range) => range,
            None => return Err(MemoryRegionBuilderError("missing field `range`".into())),
        };

        Ok(MemoryRegion {
            device,
            range,
            should_remap: self.should_remap
        })
    }
}

impl Default for MemoryRegionBuilder {
    fn default() -> Self {
        Self {
            device: None,
            range: None,
            should_remap: true,
        }
    }
}

#[derive(Debug)]
pub struct MemoryMapper {
    regions: Vec<MemoryRegion>,
}

impl MemoryMapper {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    pub fn add_region(&mut self, region: MemoryRegion) {
        self.regions.push(region)
    }

    pub fn find_region_from_addr(&self, addr: Addr) -> &MemoryRegion {
        self.regions.iter()
            .find(|region| region.range.contains(&(addr as usize)))
            .expect(&format!("no region with range containing {:#x?}", addr))
    }

    pub fn find_region_from_addr_mut(&mut self, addr: Addr) -> &mut MemoryRegion {
        self.regions.iter_mut()
            .find(|region| region.range.contains(&(addr as usize)))
            .expect(&format!("no region with range containing {:#x?}", addr))
    }
}
