use crate::prelude::*;
use std::collections::BTreeMap;

#[cfg(test)]
use hex_slice::AsHex;

#[derive(Debug)]
pub struct Cpu {
    frame_size: Short,
    mapper: MemoryMapper,
    registers: BTreeMap<RegisterVariant, Register>,
}

impl Cpu {
    #[cfg(test)]
    fn debug(&self) {
        println!("");
        for (reg, register) in self.registers.iter() {
            println!("{:?}: {:02X}", reg, &register.memory.0[..].as_hex())
        }

        let ip: Addr = self.get_register_val(RegisterVariant::Ip);

        let mut mem = Vec::with_capacity(0b10000);
        while mem.len() < mem.capacity() {
            mem.push(self.get_u8(ip + mem.len() as u16))
        }

        println!("memory: {:02X}", mem.as_hex());

        // let mut stack = Vec::with_capacity(0b1000000);
        // while stack.len() < stack.capacity() {
        //     stack.push(self.get_u8(0xffff - 1 + stack.len() as u16))
        // }

        // println!("stack: {:02X}", stack.as_hex());
    }

    fn create_registers() -> BTreeMap<RegisterVariant, Register> {
        let mut registers = BTreeMap::new();

        registers.insert(RegisterVariant::Ip, Register::new());
        registers.insert(RegisterVariant::Acc, Register::new());
        registers.insert(RegisterVariant::R1, Register::new());
        registers.insert(RegisterVariant::R2, Register::new());
        registers.insert(RegisterVariant::R3, Register::new());
        registers.insert(RegisterVariant::R4, Register::new());
        registers.insert(RegisterVariant::R5, Register::new());
        registers.insert(RegisterVariant::R6, Register::new());
        registers.insert(RegisterVariant::R7, Register::new());
        registers.insert(RegisterVariant::R8, Register::new());
        registers.insert(RegisterVariant::Sp, Register::new());
        registers.insert(RegisterVariant::Fp, Register::new());

        registers.get_mut(&RegisterVariant::Sp).unwrap()
            .set_u16(0x0000, 0xffff - 1);

        registers.get_mut(&RegisterVariant::Fp).unwrap()
            .set_u16(0x0000, 0xffff - 1);

        registers
    }

    fn get_register_val(&self, reg: RegisterVariant) -> Short {
        self.registers.get(&reg).unwrap().get_u16(0x0000)
    }

    fn fetch_register_val(&mut self) -> Short {
        let reg: RegisterVariant = self.fetch_u8().into();
        self.get_register_val(reg)
    }

    fn set_register_val(&mut self, reg: RegisterVariant, val: Short) {
        self.registers.get_mut(&reg).unwrap().set_u16(0x0000, val);
    }

    fn fetch_u8(&mut self) -> Byte {
        let ip: Addr = self.get_register_val(RegisterVariant::Ip);
        self.set_register_val(RegisterVariant::Ip, ip + 0x0001);

        self.get_u8(ip)
    }

    fn fetch_u16(&mut self) -> Short {
        let ip: Addr = self.get_register_val(RegisterVariant::Ip);
        self.set_register_val(RegisterVariant::Ip, ip + 0x0002);

        self.get_u16(ip)
    }

    fn execute(&mut self, instruction: InstructionVariant) -> bool {
        match instruction {
            InstructionVariant::MoveLitReg => {
                let val = self.fetch_u16();
                let reg: RegisterVariant = self.fetch_u8().into();
                self.set_register_val(reg, val);
            },
            InstructionVariant::MoveRegReg => {
                let val = self.fetch_register_val();
                let reg: RegisterVariant = self.fetch_u8().into();
                self.set_register_val(reg, val);
            },
            InstructionVariant::MoveRegMem => {
                let val = self.fetch_register_val();
                let addr = self.fetch_u16();
                self.set_u16(addr, val);
            },
            InstructionVariant::MoveMemReg => {
                let addr = self.fetch_u16();
                let val = self.get_u16(addr);
                let reg: RegisterVariant = self.fetch_u8().into();
                self.set_register_val(reg, val);
            },
            InstructionVariant::MoveLitMem => {
                let val = self.fetch_u16();
                let addr = self.fetch_u16();
                self.set_u16(addr, val);
            },
            InstructionVariant::MoveRegPtrReg => {
                let addr = self.fetch_register_val();
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = self.get_u16(addr);
                self.set_register_val(reg, val);
            },
            InstructionVariant::MoveLitOffReg => {
                let addr = self.fetch_u16();
                let offset = self.fetch_register_val();
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = self.get_u16(addr + offset);
                self.set_register_val(reg, val);
            },

            InstructionVariant::AddRegReg => {
                let sum = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_register_val();

                    v1 + v2
                };

                self.set_register_val(RegisterVariant::Acc, sum);
            },
            InstructionVariant::AddLitReg => {
                let sum = {
                    let v1 = self.fetch_u16();
                    let v2 = self.fetch_register_val();

                    v1 + v2
                };

                self.set_register_val(RegisterVariant::Acc, sum);
            },
            InstructionVariant::SubLitReg => {
                let diff = {
                    let v1 = self.fetch_u16();
                    let v2 = self.fetch_register_val();

                    v1 - v2
                };

                self.set_register_val(RegisterVariant::Acc, diff);
            },
            InstructionVariant::SubRegLit => {
                let diff = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_u16();

                    v1 - v2
                };

                self.set_register_val(RegisterVariant::Acc, diff);
            },
            InstructionVariant::SubRegReg => {
                let diff = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_register_val();

                    v1 - v2
                };

                self.set_register_val(RegisterVariant::Acc, diff);
            },
            InstructionVariant::IncReg => {
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = self.get_register_val(reg);
                self.set_register_val(reg, val + 1);
            },
            InstructionVariant::DecReg => {
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = self.get_register_val(reg);
                self.set_register_val(reg, val - 1);
            },
            InstructionVariant::MulLitReg => {
                let product = {
                    let v1 = self.fetch_u16();
                    let v2 = self.fetch_register_val();

                    v1 * v2
                };

                self.set_register_val(RegisterVariant::Acc, product);
            },
            InstructionVariant::MulRegReg => {
                let product = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_register_val();

                    v1 * v2
                };

                self.set_register_val(RegisterVariant::Acc, product);
            },

            InstructionVariant::LeftShiftRegLit => {
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = {
                    let v1 = self.get_register_val(reg);
                    let v2 = self.fetch_u16();

                    v1 << v2
                };

                self.set_register_val(reg, val);
            },
            InstructionVariant::LeftShiftRegReg => {
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = {
                    let v1 = self.get_register_val(reg);
                    let v2 = self.fetch_register_val();

                    v1 << v2
                };

                self.set_register_val(reg, val);
            },
            InstructionVariant::RightShiftRegLit => {
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = {
                    let v1 = self.get_register_val(reg);
                    let v2 = self.fetch_u16();

                    v1 >> v2
                };

                self.set_register_val(reg, val);
            },
            InstructionVariant::RightShiftRegReg => {
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = {
                    let v1 = self.get_register_val(reg);
                    let v2 = self.fetch_register_val();

                    v1 >> v2
                };

                self.set_register_val(reg, val);
            },
            InstructionVariant::AndRegLit => {
                let val = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_u16();

                    v1 & v2
                };

                self.set_register_val(RegisterVariant::Acc, val);
            },
            InstructionVariant::AndRegReg => {
                let val = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_register_val();

                    v1 & v2
                };

                self.set_register_val(RegisterVariant::Acc, val);
            },
            InstructionVariant::OrRegLit => {
                let val = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_u16();

                    v1 | v2
                };

                self.set_register_val(RegisterVariant::Acc, val);
            },
            InstructionVariant::OrRegReg => {
                let val = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_register_val();

                    v1 | v2
                };

                self.set_register_val(RegisterVariant::Acc, val);
            },
            InstructionVariant::XorRegLit => {
                let val = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_u16();

                    v1 ^ v2
                };

                self.set_register_val(RegisterVariant::Acc, val);
            },
            InstructionVariant::XorRegReg => {
                let val = {
                    let v1 = self.fetch_register_val();
                    let v2 = self.fetch_register_val();

                    v1 ^ v2
                };

                self.set_register_val(RegisterVariant::Acc, val);
            },
            InstructionVariant::Not => {
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = !self.get_register_val(reg);
                self.set_register_val(reg, val);
            },

            InstructionVariant::JumpNotEqReg => {
                let val = self.fetch_register_val();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val != acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpNotEqLit => {
                let val = self.fetch_u16();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val != acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpEqReg => {
                let val = self.fetch_register_val();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val == acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpEqLit => {
                let val = self.fetch_u16();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val == acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpLtReg => {
                let val = self.fetch_register_val();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val < acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpLtLit => {
                let val = self.fetch_u16();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val < acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpGtReg => {
                let val = self.fetch_register_val();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val > acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpGtLit => {
                let val = self.fetch_u16();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val > acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpLteReg => {
                let val = self.fetch_register_val();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val <= acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpLteLit => {
                let val = self.fetch_u16();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val <= acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpGteReg => {
                let val = self.fetch_register_val();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val >= acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },
            InstructionVariant::JumpGteLit => {
                let val = self.fetch_u16();
                let addr = self.fetch_u16();

                let acc = self.get_register_val(RegisterVariant::Acc);
                if val >= acc {
                    self.set_register_val(RegisterVariant::Ip, addr);
                }
            },

            InstructionVariant::PushLit => {
                let val = self.fetch_u16();
                self.stack_push(val);
            },
            InstructionVariant::PushReg => {
                let val = self.fetch_register_val();
                self.stack_push(val);
            },
            InstructionVariant::Pop => {
                let reg: RegisterVariant = self.fetch_u8().into();
                let val = self.stack_pop();
                self.set_register_val(reg, val);
            },
            InstructionVariant::CallLit => {
                let val = self.fetch_u16();

                self.stack_push_state();
                self.set_register_val(RegisterVariant::Ip, val);
            },
            InstructionVariant::CallReg => {
                let val = self.fetch_register_val();

                self.stack_push_state();
                self.set_register_val(RegisterVariant::Ip, val);
            },
            InstructionVariant::Ret => {
                self.stack_pop_state();
            },
            InstructionVariant::Halt => return true,
        }

        false
    }

    fn stack_push(&mut self, val: Short) {
        let sp: Addr = self.get_register_val(RegisterVariant::Sp);
        self.set_u16(sp, val);
        self.set_register_val(RegisterVariant::Sp, sp - 2);

        self.frame_size += 2;
    }

    fn stack_push_state(&mut self) {
        self.stack_push(self.get_register_val(RegisterVariant::R1));
        self.stack_push(self.get_register_val(RegisterVariant::R2));
        self.stack_push(self.get_register_val(RegisterVariant::R3));
        self.stack_push(self.get_register_val(RegisterVariant::R4));
        self.stack_push(self.get_register_val(RegisterVariant::R5));
        self.stack_push(self.get_register_val(RegisterVariant::R6));
        self.stack_push(self.get_register_val(RegisterVariant::R7));
        self.stack_push(self.get_register_val(RegisterVariant::R8));
        self.stack_push(self.get_register_val(RegisterVariant::Ip));

        self.stack_push(self.frame_size + 2);

        let sp: Addr = self.get_register_val(RegisterVariant::Sp);
        self.set_register_val(RegisterVariant::Fp, sp);

        self.frame_size = 0;
    }

    fn stack_pop(&mut self) -> u16 {
        let sp: Addr = self.get_register_val(RegisterVariant::Sp) + 2;
        self.set_register_val(RegisterVariant::Sp, sp);
        self.frame_size -= 2;

        self.get_u16(sp)
    }

    fn stack_pop_state(&mut self) {
        let fp: Addr = self.get_register_val(RegisterVariant::Fp);
        self.set_register_val(RegisterVariant::Sp, fp);

        let frame_size = self.stack_pop();
        self.frame_size = frame_size;

        let ip: Addr = self.stack_pop();
        let r8: Short = self.stack_pop();
        let r7: Short = self.stack_pop();
        let r6: Short = self.stack_pop();
        let r5: Short = self.stack_pop();
        let r4: Short = self.stack_pop();
        let r3: Short = self.stack_pop();
        let r2: Short = self.stack_pop();
        let r1: Short = self.stack_pop();

        self.set_register_val(RegisterVariant::Ip, ip);
        self.set_register_val(RegisterVariant::R8, r8);
        self.set_register_val(RegisterVariant::R7, r7);
        self.set_register_val(RegisterVariant::R6, r6);
        self.set_register_val(RegisterVariant::R5, r5);
        self.set_register_val(RegisterVariant::R4, r4);
        self.set_register_val(RegisterVariant::R3, r3);
        self.set_register_val(RegisterVariant::R2, r2);
        self.set_register_val(RegisterVariant::R1, r1);

        let n_args = self.stack_pop();
        for _ in 0..n_args { self.stack_pop(); }

        self.set_register_val(RegisterVariant::Fp, fp + frame_size);
    }

    pub fn step(&mut self) -> bool {
        #[cfg(test)]
        self.debug();

        let instruction: InstructionVariant = self.fetch_u8().into();
        self.execute(instruction)
    }

    pub fn run(&mut self) {
        while !self.step() {}
    }
}

impl Read for Cpu {
    fn get_u8(&self, addr: Addr) -> Byte {
        self.mapper.find_region_from_addr(addr).get_u8(addr)
    }

    fn get_u16(&self, addr: Addr) -> Short {
        self.mapper.find_region_from_addr(addr).get_u16(addr)
    }
}

impl Write for Cpu {
    fn set_u8(&mut self, addr: Addr, val: Byte) {
        self.mapper.find_region_from_addr_mut(addr).set_u8(addr, val)
    }

    fn set_u16(&mut self, addr: Addr, val: Short) {
        self.mapper.find_region_from_addr_mut(addr).set_u16(addr, val)
    }
}

impl Device for Cpu {}

impl From<Memory> for Cpu {
    fn from(memory: Memory) -> Self {
        let mut mm = MemoryMapper::new();
        mm.add_region(
            MemoryRegion::builder()
                .range(memory.get_range())
                .device(Box::new(memory))
                .finalize()
                .unwrap()
        );

        Self {
            frame_size: 0,
            mapper: mm,
            registers: Self::create_registers(),
        }
    }
}

impl From<MemoryMapper> for Cpu {
    fn from(mm: MemoryMapper) -> Self {
        Self {
            frame_size: 0,
            mapper: mm,
            registers: Self::create_registers(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::constants::*;
    use crate::registers::constants::*;

    #[test]
    fn can_fetch_u8() {
        let mut cpu = Cpu::from(Memory::with_capacity(0x100));
        assert_eq!(cpu.fetch_u8(), 0);
    }

    #[test]
    fn can_fetch_u16() {
        let mut cpu = Cpu::from(Memory::with_capacity(0x100));
        assert_eq!(cpu.fetch_u16(), 0);
    }

    #[test]
    fn can_move_lit_reg() {
        let mut memory = Memory::with_capacity(0x100);

        // move lit (0x1234) reg (r1)
        memory.set_u8(0x0000, MOV_LIT_REG);
        memory.set_u8(0x0001, 0x12);
        memory.set_u8(0x0002, 0x34);
        memory.set_u8(0x0003, R1);

        let mut cpu = Cpu::from(memory);
        cpu.step();

        cpu.debug();

        assert_eq!(cpu.get_register_val(RegisterVariant::R1), 0x1234);
    }

    #[test]
    fn can_add() {
        let mut memory = Memory::with_capacity(0x100);

        // move lit (0x1234) reg (r1)
        memory.set_u8(0x0000, MOV_LIT_REG);
        memory.set_u8(0x0001, 0x12);
        memory.set_u8(0x0002, 0x34);
        memory.set_u8(0x0003, R1);

        // move lit (0xABCD) reg (r2)
        memory.set_u8(0x0004, MOV_LIT_REG);
        memory.set_u8(0x0005, 0xAB);
        memory.set_u8(0x0006, 0xCD);
        memory.set_u8(0x0007, R2);

        // add reg (r1) reg (r2)
        memory.set_u8(0x0008, ADD_REG_REG);
        memory.set_u8(0x0009, R1);
        memory.set_u8(0x000A, R2);

        let mut cpu = Cpu::from(memory);

        cpu.step();
        cpu.step();
        cpu.step();

        cpu.debug();

        assert_eq!(cpu.get_register_val(RegisterVariant::Acc), 0xBE01);
    }

    #[test]
    fn can_move_reg_mem() {
        let mut memory = Memory::with_capacity(0x10000);

        // move lit (0x1234) reg (r1)
        memory.set_u8(0x0000, MOV_LIT_REG);
        memory.set_u8(0x0001, 0x12);
        memory.set_u8(0x0002, 0x34);
        memory.set_u8(0x0003, R1);

        // move lit (0xABCD) reg (r2)
        memory.set_u8(0x0004, MOV_LIT_REG);
        memory.set_u8(0x0005, 0xAB);
        memory.set_u8(0x0006, 0xCD);
        memory.set_u8(0x0007, R2);

        // add reg (r1) reg (r2)
        memory.set_u8(0x0008, ADD_REG_REG);
        memory.set_u8(0x0009, R1);
        memory.set_u8(0x000A, R2);

        // move reg (acc) to mem (addr 0x0100)
        memory.set_u8(0x000B, MOV_REG_MEM);
        memory.set_u8(0x000C, ACC);
        memory.set_u8(0x000D, 0x01);
        memory.set_u8(0x000E, 0x00);

        let mut cpu = Cpu::from(memory);

        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();

        cpu.debug();

        assert_eq!(cpu.get_u16(0x0100), 0xBE01);
    }

    #[test]
    fn can_jump() {
        let mut memory = Memory::with_capacity(0x10000);

        // move mem (addr 0x0100) reg (r1)
        memory.set_u8(0x0000, MOV_MEM_REG);
        memory.set_u8(0x0001, 0x01);
        memory.set_u8(0x0002, 0x00);
        memory.set_u8(0x0003, R1);

        // move lit (0x0001) reg (r2)
        memory.set_u8(0x0004, MOV_LIT_REG);
        memory.set_u8(0x0005, 0x00);
        memory.set_u8(0x0006, 0x01);
        memory.set_u8(0x0007, R2);

        // add reg (r1) reg (r2)
        memory.set_u8(0x0008, ADD_REG_REG);
        memory.set_u8(0x0009, R1);
        memory.set_u8(0x000A, R2);

        // move reg (acc) to mem (addr 0x0100)
        memory.set_u8(0x000B, MOV_REG_MEM);
        memory.set_u8(0x000C, ACC);
        memory.set_u8(0x000D, 0x01);
        memory.set_u8(0x000E, 0x00);

        // jump (addr 0x0000) if acc != lit (0x0003)
        memory.set_u8(0x000F, JNE_LIT);
        memory.set_u8(0x0010, 0x00);
        memory.set_u8(0x0011, 0x03);
        memory.set_u8(0x0012, 0x00);
        memory.set_u8(0x0013, 0x00);

        let mut cpu = Cpu::from(memory);

        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();

        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();

        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();

        cpu.debug();

        assert_eq!(cpu.get_u16(0x0100), 0x0003);
    }

    #[test]
    fn can_push_pop() {
        let mut memory = Memory::with_capacity(0x10000);

        // move lit (0x5151) reg (r1)
        memory.set_u8(0x0000, MOV_LIT_REG);
        memory.set_u8(0x0001, 0x51);
        memory.set_u8(0x0002, 0x51);
        memory.set_u8(0x0003, R1);

        // move lit (0x5151) reg (r2)
        memory.set_u8(0x0004, MOV_LIT_REG);
        memory.set_u8(0x0005, 0x42);
        memory.set_u8(0x0006, 0x42);
        memory.set_u8(0x0007, R2);

        // push reg (r1)
        memory.set_u8(0x0008, PSH_REG);
        memory.set_u8(0x0009, R1);

        // push reg (r2)
        memory.set_u8(0x000A, PSH_REG);
        memory.set_u8(0x000B, R2);

        // pop reg (r1)
        memory.set_u8(0x000C, POP);
        memory.set_u8(0x000D, R1);

        // pop reg (r2)
        memory.set_u8(0x000E, POP);
        memory.set_u8(0x000F, R2);

        let mut cpu = Cpu::from(memory);

        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();

        cpu.debug();

        assert_eq!(cpu.get_register_val(RegisterVariant::R1), 0x4242);
        assert_eq!(cpu.get_register_val(RegisterVariant::R2), 0x5151);
    }

    #[test]
    fn can_subroutine() {
        let mut memory = Memory::with_capacity(0x10000);

        // push lit (0x3333)
        memory.set_u8(0x0000, PSH_LIT);
        memory.set_u8(0x0001, 0x33);
        memory.set_u8(0x0002, 0x33);

        // push lit (0x2222)
        memory.set_u8(0x0003, PSH_LIT);
        memory.set_u8(0x0004, 0x22);
        memory.set_u8(0x0005, 0x22);

        // push lit (0x1111)
        memory.set_u8(0x0006, PSH_LIT);
        memory.set_u8(0x0007, 0x11);
        memory.set_u8(0x0008, 0x11);

        // move lit (0x1234) reg (r1)
        memory.set_u8(0x0009, MOV_LIT_REG);
        memory.set_u8(0x000A, 0x12);
        memory.set_u8(0x000B, 0x34);
        memory.set_u8(0x000C, R1);

        // move lit (0x5678) reg (r4)
        memory.set_u8(0x000D, MOV_LIT_REG);
        memory.set_u8(0x000E, 0x56);
        memory.set_u8(0x000F, 0x78);
        memory.set_u8(0x0010, R4);

        // push lit (0x0000)
        memory.set_u8(0x0011, PSH_LIT);
        memory.set_u8(0x0012, 0x00);
        memory.set_u8(0x0013, 0x00);

        // call subroutine (0x3000)
        memory.set_u8(0x0014, CAL_LIT);
        memory.set_u8(0x0015, 0x30);
        memory.set_u8(0x0016, 0x00);

        // push lit (0x4444)
        memory.set_u8(0x0017, PSH_LIT);
        memory.set_u8(0x0018, 0x44);
        memory.set_u8(0x0019, 0x44);

        // BEGIN SUBROUTINE -- ADDR 0x3000

        // push lit (0x0102)
        memory.set_u8(0x3000, PSH_LIT);
        memory.set_u8(0x3001, 0x01);
        memory.set_u8(0x3002, 0x02);

        // push lit (0x0304)
        memory.set_u8(0x3003, PSH_LIT);
        memory.set_u8(0x3004, 0x03);
        memory.set_u8(0x3005, 0x04);

        // push lit (0x0506)
        memory.set_u8(0x3006, PSH_LIT);
        memory.set_u8(0x3007, 0x05);
        memory.set_u8(0x3008, 0x06);

        // move lit (0x0708) reg (r1)
        memory.set_u8(0x3009, MOV_LIT_REG);
        memory.set_u8(0x300A, 0x07);
        memory.set_u8(0x300B, 0x08);
        memory.set_u8(0x300C, R1);

        // move lit (0x090A) reg (r8)
        memory.set_u8(0x300D, MOV_LIT_REG);
        memory.set_u8(0x300E, 0x09);
        memory.set_u8(0x300F, 0x0A);
        memory.set_u8(0x3010, R8);

        memory.set_u8(0x3011, RET);

        // END SUBROUTINE

        let mut cpu = Cpu::from(memory);

        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();

        // about to enter subroutine...
        println!("about to enter subroutine...");

        assert_eq!(cpu.get_register_val(RegisterVariant::R1), 0x1234);
        assert_eq!(cpu.get_register_val(RegisterVariant::R4), 0x5678);

        cpu.step();

        // now in subroutine
        println!("now in subroutine");

        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();

        // about to exit subroutine...
        println!("about to exit subroutine...");

        assert_eq!(cpu.get_register_val(RegisterVariant::R1), 0x0708);
        assert_eq!(cpu.get_register_val(RegisterVariant::R8), 0x090A);

        cpu.step();

        // exited subroutine
        println!("exited subroutine");

        assert_eq!(cpu.get_register_val(RegisterVariant::R1), 0x1234);
        assert_eq!(cpu.get_register_val(RegisterVariant::R4), 0x5678);

        cpu.step();
    }

    #[test]
    fn binary() {
        let bytes = include_bytes!("../tests/binary1");

        println!("{:02X?}", bytes);

        let mut memory = Memory::with_capacity(0x10000);
        memory.set_bytes(bytes);

        let mut cpu = Cpu::from(memory);

        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();

        assert_eq!(cpu.get_register_val(RegisterVariant::Acc), 0x5500);
    }

    #[test]
    fn binary_loop() {
        let bytes = include_bytes!("../tests/binary2");

        println!("{:02X?}", &bytes[..]);

        let mut memory = Memory::with_capacity(0x10000);
        memory.set_bytes(bytes);

        let mut cpu = Cpu::from(memory);

        cpu.run();

        assert_eq!(cpu.get_register_val(RegisterVariant::R2), 0x001E);
        assert_eq!(cpu.get_register_val(RegisterVariant::Ip), bytes.len() as Addr);
    }
}
