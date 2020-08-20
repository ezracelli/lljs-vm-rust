use super::*;
use combinators::*;

#[derive(Debug, PartialEq)]
pub enum Element<'a> {
    Addr(Box<Element<'a>>),
    Expr(Expr<'a>),
    Lit(Short),
    Lit8(Short),
    Reg(RegisterVariant),
    Var(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct Expr<'a> {
    pub lhs: Box<Element<'a>>,
    pub operator: Operator,
    pub rhs: Box<Element<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
pub struct OperatorParseError(u8);

impl fmt::Display for OperatorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown byte `{:02x?}`", self.0)
    }
}

impl TryFrom<u8> for Operator {
    type Error = OperatorParseError;
    fn try_from(char: u8) -> Result<Self, Self::Error> {
        match char {
            b'+' => Ok(Operator::Add),
            b'-' => Ok(Operator::Sub),
            b'*' => Ok(Operator::Mul),
            _ => Err(OperatorParseError(char)),
        }
    }
}

pub fn address<'a>() -> Parser<'a, u8, Element<'a>> {
    let addr_lit = || {
        (sym(b'&') * is_a(hex_digit).repeat(4))
            .convert(String::from_utf8)
            .convert(|hex| u16::from_str_radix(&hex, 16))
            .map(|lit| Element::Addr(Box::new(Element::Lit(lit))))
    };

    let addr_expr = || {
        (sym(b'&') * element())
            .map(|expr| Element::Addr(Box::new(expr)))
    };

    (addr_lit() | addr_expr()) - optional_whitespace()
}

pub fn element<'a>() -> Parser<'a, u8, Element<'a>> {
    (
        literal() |
        variable() |
        expr(b'[', b']')
    ) - optional_whitespace()
}

pub fn expr<'a>(open: u8, close: u8) -> Parser<'a, u8, Element<'a>> {
    let element = || {
        literal() |
        variable() |
        call(|| expr(b'(', b')'))
    };

    sym(open) * optional_whitespace() *
    (
        (
            (element() - optional_whitespace()) +
            (operator() - optional_whitespace())
        ).repeat(0..) +
        element()
    ).map(|(vec, el)| {
        let iter = vec.into_iter();

        fn get_expr<'a>(
            mut iter: std::vec::IntoIter<(Element<'a>, Operator)>,
            el: Element<'a>,
        ) -> Element<'a> {
            match iter.next() {
                Some((lhs, operator)) => Element::Expr(Expr {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(get_expr(iter, el)),
                }),
                _ => el,
            }
        }

        get_expr(iter, el)
    }) - optional_whitespace() -
    sym(close) - optional_whitespace()
}

pub fn literal<'a>() -> Parser<'a, u8, Element<'a>> {
    let lit = || {
        (sym(b'$') * is_a(hex_digit).repeat(4))
            .convert(String::from_utf8)
            .convert(|hex| u16::from_str_radix(&hex, 16))
            .map(|lit| Element::Lit(lit))
    };

    let lit8 = || {
        (sym(b'$') * is_a(hex_digit).repeat(2))
            .convert(String::from_utf8)
            .convert(|hex| u16::from_str_radix(&hex, 16))
            .map(|lit| Element::Lit8(lit))
    };

    (lit() | lit8()) - optional_whitespace()
}

pub fn operator<'a>() -> Parser<'a, u8, Operator> {
    one_of(b"+-*").convert(Operator::try_from)
}

pub fn register<'a>() -> Parser<'a, u8, Element<'a>> {
    seqi(b"ip").map(|_| Element::Reg(RegisterVariant::Ip)) |
    seqi(b"acc").map(|_| Element::Reg(RegisterVariant::Acc)) |
    seqi(b"r1").map(|_| Element::Reg(RegisterVariant::R1)) |
    seqi(b"r2").map(|_| Element::Reg(RegisterVariant::R2)) |
    seqi(b"r3").map(|_| Element::Reg(RegisterVariant::R3)) |
    seqi(b"r4").map(|_| Element::Reg(RegisterVariant::R4)) |
    seqi(b"r5").map(|_| Element::Reg(RegisterVariant::R5)) |
    seqi(b"r6").map(|_| Element::Reg(RegisterVariant::R6)) |
    seqi(b"r7").map(|_| Element::Reg(RegisterVariant::R7)) |
    seqi(b"r8").map(|_| Element::Reg(RegisterVariant::R8)) |
    seqi(b"sp").map(|_| Element::Reg(RegisterVariant::Sp)) |
    seqi(b"fp").map(|_| Element::Reg(RegisterVariant::Fp))
}

pub fn variable<'a>() -> Parser<'a, u8, Element<'a>> {
    (sym(b'!') * identifier()).map(|var| Element::Var(var))
}
