use crate::prelude::*;
use std::io::{Stdin, Stdout};

#[derive(Debug)]
pub struct ScreenDevice(Stdin, Stdout);

impl ScreenDevice {
    pub fn new() -> Self {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();

        Self(stdin, stdout)
    }
}

impl Read for ScreenDevice {
    fn get_u8(&self, _: Addr) -> Byte { unimplemented!(); }
    fn get_u16(&self, _: Addr) -> Short { unimplemented!(); }
}

impl Write for ScreenDevice {
    fn set_u8(&mut self, _: Addr, _: Byte) { unimplemented!(); }

    fn set_u16(&mut self, addr: Addr, val: Short) {
        use std::io::Write;

        // println!("{:#04x?}", val);

        let command = ((val & 0xff00) >> 0b1000) as u8;
        let char = (val & 0x00ff) as u8;

        // println!("{:#02x?} {:#02x?}", command, char);

        let x = (addr % 0x10) as u8;
        let y = (addr / 0x10) as u8;

        let mut stdout = self.1.lock();

        match command {
            0x00 => (),
            0x01 => write!(stdout, "\x1B[1m").expect("cannot write to stdout"),
            0x02 => write!(stdout, "\x1B[0m").expect("cannot write to stdout"),
            0xff => write!(stdout, "\x1B[2J").expect("cannot write to stdout"),
            _ => unimplemented!("unknown command `{:#02x?}`", command),
        }

        // move to x, y
        write!(stdout, "\x1B[{};{}H", y + 1, (x + 1) * 2).expect("cannot write to stdout");

        // write char
        stdout.write(&[char]).expect("cannot write to stdout");

        stdout.flush().unwrap();
    }
}

impl Device for ScreenDevice {}
