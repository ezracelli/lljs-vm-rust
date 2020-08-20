use vm::prelude::*;
use vm_assembler::{assemble, parse};

// #[test]
// fn bracketed_expr() {
//     let input = b"start:\n\tmov [(!loc - $ffff) + ($1234 * ($0001 * $0001) + !loc)], R1";
//     let res = parser::parse(input)
//         .expect("could not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Expr(Expr {
//                     lhs: Box::new(Element::Expr(Expr {
//                         lhs: Box::new(Element::Var("loc")),
//                         operator: Operator::Sub,
//                         rhs: Box::new(Element::Lit(0xffff)),
//                     })),
//                     operator: Operator::Add,
//                     rhs: Box::new(Element::Expr(Expr {
//                         lhs: Box::new(Element::Lit(0x1234)),
//                         operator: Operator::Mul,
//                         rhs: Box::new(Element::Expr(Expr {
//                             lhs: Box::new(Element::Expr(Expr {
//                                 lhs: Box::new(Element::Lit(0x0001)),
//                                 operator: Operator::Mul,
//                                 rhs: Box::new(Element::Lit(0x0001)),
//                             })),
//                             operator: Operator::Add,
//                             rhs: Box::new(Element::Var("loc"))
//                         })),
//                     })),
//                 }),
//                 Element::Reg(RegisterVariant::R1)
//             ],
//             variant: InstructionVariant::MoveLitReg,
//         }),
//     ]);
// }

// // #[test]
// // fn bracketed_expr_ooo() {
// //     let input = b"start:\n\tmov [!loc - $0001 * $0002], R1";
// //     let res = parser::parse(input)
// //         .expect("could not parse");

// //     assert_eq!(res, vec![ Line::Instruction(Instruction {
// //         arguments: vec![ Element::Expr(Expr {
// //             lhs: Box::new(Element::Var("loc")),
// //             operator: Operator::Sub,
// //             rhs: Box::new(Element::Expr(Expr {
// //                 lhs: Box::new(Element::Lit(0x0001)),
// //                 operator: Operator::Mul,
// //                 rhs: Box::new(Element::Lit(0x0002)),
// //             })),
// //         }), Element::Reg(RegisterVariant::R1) ],
// //         variant: InstructionVariant::MoveLitReg,
// //     }) ]);

// //     let input = b"start:\n\tmov [!loc * $0001 - $0002], R1";
// //     let res = parser::parse(input)
// //         .expect("could not parse");

// //     assert_eq!(res, vec![ Line::Instruction(Instruction {
// //         arguments: vec![ Element::Expr(Expr {
// //             lhs: Box::new(Element::Expr(Expr {
// //                 lhs: Box::new(Element::Var("loc")),
// //                 operator: Operator::Mul,
// //                 rhs: Box::new(Element::Lit(0x0001)),
// //             })),
// //             operator: Operator::Sub,
// //             rhs: Box::new(Element::Lit(0x0002)),
// //         }), Element::Reg(RegisterVariant::R1) ],
// //         variant: InstructionVariant::MoveLitReg,
// //     }) ]);
// // }

// #[test]
// fn move_lit_reg() {
//     let res = parser::parse(b"start:\n\tmov $1234, R1")
//         .expect("coult not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Lit(0x1234),
//                 Element::Reg(RegisterVariant::R1),
//             ],
//             variant: InstructionVariant::MoveLitReg,
//         }),
//     ]);
// }

// #[test]
// fn move_reg_reg() {
//     let res = parser::parse(b"start:\n\tmov acc, r1")
//         .expect("coult not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Reg(RegisterVariant::Acc),
//                 Element::Reg(RegisterVariant::R1),
//             ],
//             variant: InstructionVariant::MoveRegReg,
//         }),
//     ]);
// }

// #[test]
// fn move_reg_mem() {
//     let res = parser::parse(b"start:\n\tmov R2, &C0DE")
//         .expect("coult not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Reg(RegisterVariant::R2),
//                 Element::Addr(Box::new(Element::Lit(0xC0DE))),
//             ],
//             variant: InstructionVariant::MoveRegMem,
//         }),
//     ]);
// }

// #[test]
// fn move_mem_reg() {
//     let res = parser::parse(b"start:\n\tmov &[!loc + $0000], R8")
//         .expect("coult not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Addr(Box::new(Element::Expr(Expr {
//                     lhs: Box::new(Element::Var("loc")),
//                     operator: Operator::Add,
//                     rhs: Box::new(Element::Lit(0x0000)),
//                 }))),
//                 Element::Reg(RegisterVariant::R8),
//             ],
//             variant: InstructionVariant::MoveMemReg,
//         }),
//     ]);
// }

// #[test]
// fn move_lit_mem() {
//     let res = parser::parse(b"start:\n\tmov [!loc + $0000], &0000")
//         .expect("coult not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Expr(Expr {
//                     lhs: Box::new(Element::Var("loc")),
//                     operator: Operator::Add,
//                     rhs: Box::new(Element::Lit(0x0000)),
//                 }),
//                 Element::Addr(Box::new(Element::Lit(0x0000))),
//             ],
//             variant: InstructionVariant::MoveLitMem,
//         }),
//     ]);
// }

// #[test]
// fn move_reg_ptr_reg() {
//     let res = parser::parse(b"start:\n\tmov &acc, r1")
//         .expect("coult not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Addr(Box::new(Element::Reg(RegisterVariant::Acc))),
//                 Element::Reg(RegisterVariant::R1),
//             ],
//             variant: InstructionVariant::MoveRegPtrReg,
//         }),
//     ]);
// }

// #[test]
// fn move_lit_offset_reg() {
//     let res = parser::parse(b"start:\n\tmov $c0de, &acc, r1")
//         .expect("coult not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Lit(0xC0DE),
//                 Element::Addr(Box::new(Element::Reg(RegisterVariant::Acc))),
//                 Element::Reg(RegisterVariant::R1),
//             ],
//             variant: InstructionVariant::MoveLitOffReg,
//         }),
//     ]);
// }

// #[test]
// fn file() {
//     let input = include_bytes!("assembly1.asm");
//     let res = parser::parse(input)
//         .expect("coult not parse");

//     assert_eq!(res, vec![
//         Line::Label("start"),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Lit(0x4200),
//                 Element::Reg(RegisterVariant::R1),
//             ],
//             variant: InstructionVariant::MoveLitReg,
//         }),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Reg(RegisterVariant::R1),
//                 Element::Addr(Box::new(Element::Lit(0x0060))),
//             ],
//             variant: InstructionVariant::MoveRegMem,
//         }),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Lit(0x1300),
//                 Element::Reg(RegisterVariant::R1),
//             ],
//             variant: InstructionVariant::MoveLitReg,
//         }),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Addr(Box::new(Element::Lit(0x0060))),
//                 Element::Reg(RegisterVariant::R2),
//             ],
//             variant: InstructionVariant::MoveMemReg,
//         }),
//         Line::Instruction(Instruction {
//             arguments: vec![
//                 Element::Reg(RegisterVariant::R1),
//                 Element::Reg(RegisterVariant::R2),
//             ],
//             variant: InstructionVariant::AddRegReg,
//         }),
//     ]);
// }

#[test]
fn assembler() {
    let input = include_bytes!("assembly1.asm");
    let res = parse(input)
        .expect("coult not parse");

    let bytes = assemble(res);

    assert_eq!(bytes, vec![
        InstructionVariant::MoveLitReg.into(),
            0x42, 0x00,
            RegisterVariant::R1.into(),
        InstructionVariant::MoveRegMem.into(),
            RegisterVariant::R1.into(),
            0x00, 0x60,
        InstructionVariant::MoveLitReg.into(),
            0x13, 0x00,
            RegisterVariant::R1.into(),
        InstructionVariant::MoveMemReg.into(),
            0x00, 0x60,
            RegisterVariant::R2.into(),
        InstructionVariant::AddRegReg.into(),
            RegisterVariant::R1.into(),
            RegisterVariant::R2.into(),

        InstructionVariant::Halt.into(),
    ]);
}

#[test]
fn assembler_vars() {
    let input = include_bytes!("assembly2.asm");
    let res = parse(input)
        .expect("coult not parse");

    let bytes = assemble(res);

    assert_eq!(bytes, vec![
        InstructionVariant::MoveLitMem.into(),
            0x0, 0x0A,
            0x00, 0x50,

        InstructionVariant::MoveMemReg.into(),
            0x00, 0x50,
            RegisterVariant::Acc.into(),
        InstructionVariant::DecReg.into(),
            RegisterVariant::Acc.into(),
        InstructionVariant::MoveRegMem.into(),
            RegisterVariant::Acc.into(),
            0x00, 0x50,
        InstructionVariant::IncReg.into(),
            RegisterVariant::R2.into(),
        InstructionVariant::IncReg.into(),
            RegisterVariant::R2.into(),
        InstructionVariant::IncReg.into(),
            RegisterVariant::R2.into(),
        InstructionVariant::JumpNotEqLit.into(),
            0x00, 0x00,
            0x00, 0x05,

        InstructionVariant::Halt.into(),
    ]);
}
