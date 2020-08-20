use super::*;

pub fn identifier<'a>() -> Parser<'a, u8, &'a str> {
    ((is_a(alpha) | sym(b'_')) + (is_a(alphanum) | sym(b'_')).repeat(0..))
        .collect()
        .convert(std::str::from_utf8)
}

pub fn newline<'a>() -> Parser<'a, u8, ()> {
    sym(b'\n').discard() | seq(b"\r\n").discard()
}

pub fn seqi<'a>(seq: &'static [u8]) -> Parser<'a, u8, Vec<u8>> {
    let seq = std::str::from_utf8(seq).unwrap();

    take(seq.len()).convert(std::str::from_utf8).convert(move |str| {
        if str.to_lowercase() == seq {
            Ok(str.bytes().collect::<Vec<u8>>())
        } else { Err("") }
    })
}

pub fn whitespace<'a>() -> Parser<'a, u8, ()> {
    (is_a(space) | sym(b'\t')).repeat(1..).discard()
}

pub fn optional_whitespace<'a>() -> Parser<'a, u8, ()> {
    (is_a(space) | sym(b'\t')).repeat(0..).discard()
}
