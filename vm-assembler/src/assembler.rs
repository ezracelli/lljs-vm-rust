use crate::parser::{Element, Line, Operator};
use std::collections::HashMap;
use vm::prelude::*;

struct State<'a> {
    labels: HashMap<&'a str, Addr>,
    out: Vec<Byte>,
}

impl<'a> State<'a> {
    fn push_byte(&mut self, byte: Byte) {
        self.out.push(byte);
    }

    fn push_bytes(&mut self, bytes: Vec<Byte>) {
        self.out.extend_from_slice(&bytes);
    }

    fn insert_label(&mut self, label: &'a str) {
        self.labels.insert(label, self.out.len() as Addr);
    }

    fn evaluate(&mut self, element: Element) -> Vec<Byte> {
        match element {
            Element::Addr(addr) => self.evaluate(*addr),
            Element::Expr(expr) => {
                let convert_to_short = |bytes: Vec<u8>| -> Short {
                    bytes
                        .iter()
                        .rev()
                        .enumerate()
                        .fold(0, |prev, (i, curr)| {
                            prev + ((*curr as Short) << (i * 0x100))
                        })
                };

                let lhs = convert_to_short(self.evaluate(*expr.lhs));
                let rhs = convert_to_short(self.evaluate(*expr.rhs));

                let res = match expr.operator {
                    Operator::Add => lhs + rhs,
                    Operator::Sub => lhs - rhs,
                    Operator::Mul => lhs * rhs,
                };

                vec![
                    (res / 0x100) as Byte,
                    (res % 0x100) as Byte,
                ]
            }
            Element::Lit(lit) => vec![
                (lit / 0x100) as Byte,
                (lit % 0x100) as Byte,
            ],
            Element::Lit8(lit) => vec![ 0x00, (lit % 0x100) as Byte ],
            Element::Reg(reg) => vec![ reg.into() ],
            Element::Var(var) => {
                let addr = self.labels.get(var).expect("label does not exist");

                vec![
                    (addr / 0x100) as Byte,
                    (addr % 0x100) as Byte,
                ]
            }
        }
    }
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            labels: HashMap::new(),
            out: Vec::new(),
        }
    }
}

pub fn assemble<'a>(parsed: Vec<Line<'a>>) -> Vec<Byte> {
    let mut state = State::default();

    for line in parsed {
        match line {
            Line::Instruction(instruction) => {
                let variant = instruction.variant;
                state.push_byte(variant.into());

                for argument in instruction.arguments {
                    let bytes = state.evaluate(argument);
                    state.push_bytes(bytes);
                }
            },
            Line::Label(label) => {
                state.insert_label(label);
            },
        }
    }

    state.out
}
