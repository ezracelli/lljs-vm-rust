use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionVariant {
    MoveLitReg,
    MoveRegReg,
    MoveRegMem,
    MoveMemReg,
    MoveLitMem,
    MoveRegPtrReg,
    MoveLitOffReg,

    AddRegReg,
    AddLitReg,
    SubLitReg,
    SubRegLit,
    SubRegReg,
    IncReg,
    DecReg,
    MulLitReg,
    MulRegReg,

    LeftShiftRegLit,
    LeftShiftRegReg,
    RightShiftRegLit,
    RightShiftRegReg,
    AndRegLit,
    AndRegReg,
    OrRegLit,
    OrRegReg,
    XorRegLit,
    XorRegReg,
    Not,

    JumpNotEqReg,
    JumpNotEqLit,
    JumpEqReg,
    JumpEqLit,
    JumpLtReg,
    JumpLtLit,
    JumpGtReg,
    JumpGtLit,
    JumpLteReg,
    JumpLteLit,
    JumpGteReg,
    JumpGteLit,

    PushLit,
    PushReg,
    Pop,
    CallLit,
    CallReg,
    Ret,
    Halt,
}

impl InstructionVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MoveLitReg => "mov",
            Self::MoveRegReg => "mov",
            Self::MoveRegMem => "mov",
            Self::MoveMemReg => "mov",
            Self::MoveLitMem => "mov",
            Self::MoveRegPtrReg => "mov",
            Self::MoveLitOffReg => "mov",

            Self::AddRegReg => "add",
            Self::AddLitReg => "add",
            Self::SubLitReg => "sub",
            Self::SubRegLit => "sub",
            Self::SubRegReg => "sub",
            Self::IncReg => "inc",
            Self::DecReg => "dec",
            Self::MulLitReg => "mul",
            Self::MulRegReg => "mul",

            Self::LeftShiftRegLit => "lsh",
            Self::LeftShiftRegReg => "lsh",
            Self::RightShiftRegLit => "rsh",
            Self::RightShiftRegReg => "rsh",
            Self::AndRegLit => "and",
            Self::AndRegReg => "and",
            Self::OrRegLit => "or",
            Self::OrRegReg => "or",
            Self::XorRegLit => "xor",
            Self::XorRegReg => "xor",
            Self::Not => "not",

            Self::JumpNotEqReg => "jne",
            Self::JumpNotEqLit => "jne",
            Self::JumpEqReg => "jeq",
            Self::JumpEqLit => "jeq",
            Self::JumpLtReg => "jlt",
            Self::JumpLtLit => "jlt",
            Self::JumpGtReg => "jgt",
            Self::JumpGtLit => "jgt",
            Self::JumpLteReg => "jle",
            Self::JumpLteLit => "jle",
            Self::JumpGteReg => "jge",
            Self::JumpGteLit => "jge",

            Self::PushLit => "psh",
            Self::PushReg => "psh",
            Self::Pop => "pop",
            Self::CallLit => "cal",
            Self::CallReg => "cal",
            Self::Ret => "ret",
            Self::Halt => "hlt",
        }
    }
}

impl From<Byte> for InstructionVariant {
    fn from(i: Byte) -> Self {
        match i {
            constants::MOV_LIT_REG => Self::MoveLitReg,
            constants::MOV_REG_REG => Self::MoveRegReg,
            constants::MOV_REG_MEM => Self::MoveRegMem,
            constants::MOV_MEM_REG => Self::MoveMemReg,
            constants::MOV_LIT_MEM => Self::MoveLitMem,
            constants::MOV_REG_PTR_REG => Self::MoveRegPtrReg,
            constants::MOV_LIT_OFF_REG => Self::MoveLitOffReg,

            constants::ADD_REG_REG => Self::AddRegReg,
            constants::ADD_LIT_REG => Self::AddLitReg,
            constants::SUB_LIT_REG => Self::SubLitReg,
            constants::SUB_REG_LIT => Self::SubRegLit,
            constants::SUB_REG_REG => Self::SubRegReg,
            constants::INC_REG => Self::IncReg,
            constants::DEC_REG => Self::DecReg,
            constants::MUL_LIT_REG => Self::MulLitReg,
            constants::MUL_REG_REG => Self::MulRegReg,

            constants::LSF_REG_LIT => Self::LeftShiftRegLit,
            constants::LSF_REG_REG => Self::LeftShiftRegReg,
            constants::RSF_REG_LIT => Self::RightShiftRegLit,
            constants::RSF_REG_REG => Self::RightShiftRegReg,
            constants::AND_REG_LIT => Self::AndRegLit,
            constants::AND_REG_REG => Self::AndRegReg,
            constants::OR_REG_LIT => Self::OrRegLit,
            constants::OR_REG_REG => Self::OrRegReg,
            constants::XOR_REG_LIT => Self::XorRegLit,
            constants::XOR_REG_REG => Self::XorRegReg,
            constants::NOT => Self::Not,

            constants::JNE_REG => Self::JumpNotEqReg,
            constants::JNE_LIT => Self::JumpNotEqLit,
            constants::JEQ_REG => Self::JumpEqReg,
            constants::JEQ_LIT => Self::JumpEqLit,
            constants::JLT_REG => Self::JumpLtReg,
            constants::JLT_LIT => Self::JumpLtLit,
            constants::JGT_REG => Self::JumpGtReg,
            constants::JGT_LIT => Self::JumpGtLit,
            constants::JLE_REG => Self::JumpLteReg,
            constants::JLE_LIT => Self::JumpLteLit,
            constants::JGE_REG => Self::JumpGteReg,
            constants::JGE_LIT => Self::JumpGteLit,

            constants::PSH_LIT => Self::PushLit,
            constants::PSH_REG => Self::PushReg,
            constants::POP => Self::Pop,
            constants::CAL_LIT => Self::CallLit,
            constants::CAL_REG => Self::CallReg,
            constants::RET => Self::Ret,
            constants::HLT => Self::Halt,
            _ => unimplemented!("unknown instruction `{:#x?}`", i),
        }
    }
}

impl Into<Byte> for InstructionVariant {
    fn into(self) -> Byte {
        match self {
            Self::MoveLitReg => constants::MOV_LIT_REG,
            Self::MoveRegReg => constants::MOV_REG_REG,
            Self::MoveRegMem => constants::MOV_REG_MEM,
            Self::MoveMemReg => constants::MOV_MEM_REG,
            Self::MoveLitMem => constants::MOV_LIT_MEM,
            Self::MoveRegPtrReg => constants::MOV_REG_PTR_REG,
            Self::MoveLitOffReg => constants::MOV_LIT_OFF_REG,

            Self::AddRegReg => constants::ADD_REG_REG,
            Self::AddLitReg => constants::ADD_LIT_REG,
            Self::SubLitReg => constants::SUB_LIT_REG,
            Self::SubRegLit => constants::SUB_REG_LIT,
            Self::SubRegReg => constants::SUB_REG_REG,
            Self::IncReg => constants::INC_REG,
            Self::DecReg => constants::DEC_REG,
            Self::MulLitReg => constants::MUL_LIT_REG,
            Self::MulRegReg => constants::MUL_REG_REG,

            Self::LeftShiftRegLit => constants::LSF_REG_LIT,
            Self::LeftShiftRegReg => constants::LSF_REG_REG,
            Self::RightShiftRegLit => constants::RSF_REG_LIT,
            Self::RightShiftRegReg => constants::RSF_REG_REG,
            Self::AndRegLit => constants::AND_REG_LIT,
            Self::AndRegReg => constants::AND_REG_REG,
            Self::OrRegLit => constants::OR_REG_LIT,
            Self::OrRegReg => constants::OR_REG_REG,
            Self::XorRegLit => constants::XOR_REG_LIT,
            Self::XorRegReg => constants::XOR_REG_REG,
            Self::Not => constants::NOT,

            Self::JumpNotEqReg => constants::JNE_REG,
            Self::JumpNotEqLit => constants::JNE_LIT,
            Self::JumpEqReg => constants::JEQ_REG,
            Self::JumpEqLit => constants::JEQ_LIT,
            Self::JumpLtReg => constants::JLT_REG,
            Self::JumpLtLit => constants::JLT_LIT,
            Self::JumpGtReg => constants::JGT_REG,
            Self::JumpGtLit => constants::JGT_LIT,
            Self::JumpLteReg => constants::JLE_REG,
            Self::JumpLteLit => constants::JLE_LIT,
            Self::JumpGteReg => constants::JGE_REG,
            Self::JumpGteLit => constants::JGE_LIT,

            Self::PushLit => constants::PSH_LIT,
            Self::PushReg => constants::PSH_REG,
            Self::Pop => constants::POP,
            Self::CallLit => constants::CAL_LIT,
            Self::CallReg => constants::CAL_REG,
            Self::Ret => constants::RET,
            Self::Halt => constants::HLT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionArguments {
    None,
    Reg,
    Lit,
    LitReg,
    RegReg,
    RegLit,
    RegMem,
    MemReg,
    LitMem,
    RegPtrReg,
    LitOffReg,
}

impl InstructionArguments {
    pub fn bytes(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Reg => 1,
            Self::Lit => 2,
            Self::LitReg => 3,
            Self::RegReg => 2,
            Self::RegLit => 3,
            Self::RegMem => 3,
            Self::MemReg => 3,
            Self::LitMem => 4,
            Self::RegPtrReg => 2,
            Self::LitOffReg => 4,
        }
    }
}

impl From<InstructionVariant> for InstructionArguments {
    fn from(variant: InstructionVariant) -> Self {
        match variant {
            InstructionVariant::MoveLitReg => Self::LitReg,
            InstructionVariant::MoveRegReg => Self::RegReg,
            InstructionVariant::MoveRegMem => Self::RegMem,
            InstructionVariant::MoveMemReg => Self::MemReg,
            InstructionVariant::MoveLitMem => Self::LitMem,
            InstructionVariant::MoveRegPtrReg => Self::RegPtrReg,
            InstructionVariant::MoveLitOffReg => Self::LitOffReg,

            InstructionVariant::AddRegReg => Self::RegReg,
            InstructionVariant::AddLitReg => Self::LitReg,
            InstructionVariant::SubLitReg => Self::LitReg,
            InstructionVariant::SubRegLit => Self::RegLit,
            InstructionVariant::SubRegReg => Self::RegReg,
            InstructionVariant::IncReg => Self::Reg,
            InstructionVariant::DecReg => Self::Reg,
            InstructionVariant::MulLitReg => Self::LitReg,
            InstructionVariant::MulRegReg => Self::RegReg,

            InstructionVariant::LeftShiftRegLit => Self::RegLit,
            InstructionVariant::LeftShiftRegReg => Self::RegReg,
            InstructionVariant::RightShiftRegLit => Self::RegLit,
            InstructionVariant::RightShiftRegReg => Self::RegReg,
            InstructionVariant::AndRegLit => Self::RegLit,
            InstructionVariant::AndRegReg => Self::RegReg,
            InstructionVariant::OrRegLit => Self::RegLit,
            InstructionVariant::OrRegReg => Self::RegReg,
            InstructionVariant::XorRegLit => Self::RegLit,
            InstructionVariant::XorRegReg => Self::RegReg,
            InstructionVariant::Not => Self::Reg,

            InstructionVariant::JumpNotEqReg => Self::RegMem,
            InstructionVariant::JumpNotEqLit => Self::LitMem,
            InstructionVariant::JumpEqReg => Self::RegMem,
            InstructionVariant::JumpEqLit => Self::LitMem,
            InstructionVariant::JumpLtReg => Self::RegMem,
            InstructionVariant::JumpLtLit => Self::LitMem,
            InstructionVariant::JumpGtReg => Self::RegMem,
            InstructionVariant::JumpGtLit => Self::LitMem,
            InstructionVariant::JumpLteReg => Self::RegMem,
            InstructionVariant::JumpLteLit => Self::LitMem,
            InstructionVariant::JumpGteReg => Self::RegMem,
            InstructionVariant::JumpGteLit => Self::LitMem,

            InstructionVariant::PushLit => Self::Lit,
            InstructionVariant::PushReg => Self::Reg,
            InstructionVariant::Pop => Self::Reg,
            InstructionVariant::CallLit => Self::Lit,
            InstructionVariant::CallReg => Self::Reg,
            InstructionVariant::Ret => Self::None,
            InstructionVariant::Halt => Self::None,
        }
    }
}


pub mod constants {
    use super::*;

    pub const MOV_LIT_REG: Byte = 0x10;
    pub const MOV_REG_REG: Byte = 0x11;
    pub const MOV_REG_MEM: Byte = 0x12;
    pub const MOV_MEM_REG: Byte = 0x13;
    pub const MOV_LIT_MEM: Byte = 0x1B;
    pub const MOV_REG_PTR_REG: Byte = 0x1C;
    pub const MOV_LIT_OFF_REG: Byte = 0x1D;

    pub const ADD_REG_REG: Byte = 0x14;
    pub const ADD_LIT_REG: Byte = 0x3F;
    pub const SUB_LIT_REG: Byte = 0x16;
    pub const SUB_REG_LIT: Byte = 0x1E;
    pub const SUB_REG_REG: Byte = 0x1F;
    pub const INC_REG: Byte = 0x35;
    pub const DEC_REG: Byte = 0x36;
    pub const MUL_LIT_REG: Byte = 0x20;
    pub const MUL_REG_REG: Byte = 0x21;

    pub const LSF_REG_LIT: Byte = 0x26;
    pub const LSF_REG_REG: Byte = 0x27;
    pub const RSF_REG_LIT: Byte = 0x2A;
    pub const RSF_REG_REG: Byte = 0x2B;
    pub const AND_REG_LIT: Byte = 0x2E;
    pub const AND_REG_REG: Byte = 0x2F;
    pub const OR_REG_LIT: Byte = 0x30;
    pub const OR_REG_REG: Byte = 0x31;
    pub const XOR_REG_LIT: Byte = 0x32;
    pub const XOR_REG_REG: Byte = 0x33;
    pub const NOT: Byte = 0x34;

    pub const JNE_REG: Byte = 0x40;
    pub const JNE_LIT: Byte = 0x15;
    pub const JEQ_REG: Byte = 0x3E;
    pub const JEQ_LIT: Byte = 0x41;
    pub const JLT_REG: Byte = 0x42;
    pub const JLT_LIT: Byte = 0x43;
    pub const JGT_REG: Byte = 0x44;
    pub const JGT_LIT: Byte = 0x45;
    pub const JLE_REG: Byte = 0x46;
    pub const JLE_LIT: Byte = 0x47;
    pub const JGE_REG: Byte = 0x48;
    pub const JGE_LIT: Byte = 0x49;

    pub const PSH_LIT: Byte = 0x17;
    pub const PSH_REG: Byte = 0x18;
    pub const POP: Byte = 0x1A;
    pub const CAL_LIT: Byte = 0x5E;
    pub const CAL_REG: Byte = 0x5F;
    pub const RET: Byte = 0x60;
    pub const HLT: Byte = 0xFF;
}
