#![no_std]

pub mod pal;

use pal::Console;

pub fn run<C: Console>() {
    C::putc(b'H');
    C::putc(b'e');
    C::putc(b'l');
    C::putc(b'l');
    C::putc(b'o');
    C::putc(b',');
    C::putc(b' ');
    C::putc(b'w');
    C::putc(b'o');
    C::putc(b'r');
    C::putc(b'l');
    C::putc(b'd');
    C::putc(b'!');
}
