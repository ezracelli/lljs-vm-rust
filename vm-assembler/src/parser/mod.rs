use pom::char_class::*;
use pom::parser::*;
use std::convert::TryFrom;
use std::fmt;
use vm::prelude::*;

mod arguments;
mod combinators;
mod instructions;

// use arguments::*;
use combinators::*;
use instructions::*;

pub use arguments::{
    Element,
    Expr,
    Operator,
    OperatorParseError,
};

pub use instructions::Instruction;

#[derive(Debug, PartialEq)]
pub enum Line<'a> {
    Instruction(Instruction<'a>),
    Label(&'a str),
}

fn line<'a>() -> Parser<'a, u8, Instruction<'a>> {
    instruction(InstructionVariant::MoveLitReg) |
    instruction(InstructionVariant::MoveRegReg) |
    instruction(InstructionVariant::MoveRegMem) |
    instruction(InstructionVariant::MoveMemReg) |
    instruction(InstructionVariant::MoveLitMem) |
    instruction(InstructionVariant::MoveRegPtrReg) |
    instruction(InstructionVariant::MoveLitOffReg) |

    instruction(InstructionVariant::AddRegReg) |
    instruction(InstructionVariant::AddLitReg) |
    instruction(InstructionVariant::SubLitReg) |
    instruction(InstructionVariant::SubRegLit) |
    instruction(InstructionVariant::SubRegReg) |
    instruction(InstructionVariant::IncReg) |
    instruction(InstructionVariant::DecReg) |
    instruction(InstructionVariant::MulLitReg) |
    instruction(InstructionVariant::MulRegReg) |

    instruction(InstructionVariant::LeftShiftRegLit) |
    instruction(InstructionVariant::LeftShiftRegReg) |
    instruction(InstructionVariant::RightShiftRegLit) |
    instruction(InstructionVariant::RightShiftRegReg) |
    instruction(InstructionVariant::AndRegLit) |
    instruction(InstructionVariant::AndRegReg) |
    instruction(InstructionVariant::OrRegLit) |
    instruction(InstructionVariant::OrRegReg) |
    instruction(InstructionVariant::XorRegLit) |
    instruction(InstructionVariant::XorRegReg) |
    instruction(InstructionVariant::Not) |

    instruction(InstructionVariant::JumpNotEqReg) |
    instruction(InstructionVariant::JumpNotEqLit) |
    instruction(InstructionVariant::JumpEqReg) |
    instruction(InstructionVariant::JumpEqLit) |
    instruction(InstructionVariant::JumpLtReg) |
    instruction(InstructionVariant::JumpLtLit) |
    instruction(InstructionVariant::JumpGtReg) |
    instruction(InstructionVariant::JumpGtLit) |
    instruction(InstructionVariant::JumpLteReg) |
    instruction(InstructionVariant::JumpLteLit) |
    instruction(InstructionVariant::JumpGteReg) |
    instruction(InstructionVariant::JumpGteLit) |

    instruction(InstructionVariant::PushLit) |
    instruction(InstructionVariant::PushReg) |
    instruction(InstructionVariant::Pop) |
    instruction(InstructionVariant::CallLit) |
    instruction(InstructionVariant::CallReg) |
    instruction(InstructionVariant::Ret) |
    instruction(InstructionVariant::Halt)
}

pub fn parse<'a>(input: &'a [u8]) -> pom::Result<Vec<Line<'a>>> {
    let parser = || {
        (
            (identifier() - sym(b':') - optional_whitespace()).map(|s| Line::Label(s)) |
            (sym(b'\t') * line()).map(|i| Line::Instruction(i))
        ) - (
            (optional_whitespace() * (newline() | end())) *
            (optional_whitespace() * newline()).repeat(0..)
        )
    };

    (parser().repeat(0..) - end()).parse(input)
}
