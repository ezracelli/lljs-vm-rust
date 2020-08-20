use super::*;
use arguments::*;
use combinators::*;

#[derive(Debug, PartialEq)]
pub struct Instruction<'a> {
    pub arguments: Vec<Element <'a>>,
    pub variant: InstructionVariant,
}

pub fn instruction<'a>(variant: InstructionVariant) -> Parser<'a, u8, Instruction<'a>> {
    // println!("{}, {:?}", variant.as_str(), InstructionArguments::from(variant));
    seqi(variant.as_str().as_bytes()) *
    (match InstructionArguments::from(variant) {
        InstructionArguments::None => none(),
        InstructionArguments::Reg => whitespace() * reg(),
        InstructionArguments::Lit => whitespace() * lit(),
        InstructionArguments::LitReg => whitespace() * lit_reg(),
        InstructionArguments::RegReg => whitespace() * reg_reg(),
        InstructionArguments::RegLit => whitespace() * reg_lit(),
        InstructionArguments::RegMem => whitespace() * reg_mem(),
        InstructionArguments::MemReg => whitespace() * mem_reg(),
        InstructionArguments::LitMem => whitespace() * lit_mem(),
        InstructionArguments::RegPtrReg => whitespace() * reg_ptr_reg(),
        InstructionArguments::LitOffReg => whitespace() * lit_off_reg(),
    }).map(move |arguments| Instruction {
        arguments,
        variant,
    })
}

fn none<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        optional_whitespace()
    ).map(|()| vec![])
}

fn reg<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (register() - optional_whitespace())
    ).map(|reg| vec![ reg ])
}

fn lit<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (element() - optional_whitespace())
    ).map(|lit| vec![ lit ])
}

fn lit_reg<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (element() - sym(b',') - optional_whitespace()) +
        (register() - optional_whitespace())
    ).map(|(lit, reg)| vec![ lit, reg ])
}

fn reg_reg<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (register() - sym(b',') - optional_whitespace()) +
        (register() - optional_whitespace())
    ).map(|(reg1, reg2)| vec![ reg1, reg2 ])
}

fn reg_lit<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (register() - sym(b',') - optional_whitespace()) +
        (element() - optional_whitespace())
    ).map(|(reg1, reg2)| vec![ reg1, reg2 ])
}

fn reg_mem<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (register() - sym(b',') - optional_whitespace()) +
        ((
            address() |
            (sym(b'&') * element())
        ) - optional_whitespace())
    ).map(|(reg, addr)| vec![ reg, addr ])
}

fn mem_reg<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (address() - sym(b',') - optional_whitespace()) +
        (register() - optional_whitespace())
    ).map(|(addr, reg)| vec![ addr, reg ])
}

fn lit_mem<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (element() - sym(b',') - optional_whitespace()) +
        (address() - optional_whitespace())
    ).map(|(lit, addr)| vec![ lit, addr ])
}

fn reg_ptr_reg<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (sym(b'&') * register() - sym(b',') - optional_whitespace()) +
        (register() - optional_whitespace())
    ).map(|(reg1, reg2)| {
        vec![ Element::Addr(Box::new(reg1)), reg2 ]
    })
}

fn lit_off_reg<'a>() -> Parser<'a, u8, Vec<Element <'a>>> {
    (
        (element() - sym(b',') - optional_whitespace()) +
        (sym(b'&') * register() - sym(b',') - optional_whitespace()) +
        (register() - optional_whitespace())
    )
        .map(|((lit, reg1), reg2)| {
            vec![ lit, Element::Addr(Box::new(reg1)), reg2 ]
        })
}
