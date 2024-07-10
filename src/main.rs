#![no_std]
#![no_main]

mod panic;
mod arch;
mod bsp;

use core::arch::global_asm;
global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn main() {
    use bsp::pl011::Write;
    let mut UART0 = bsp::pl011::UART(0x0900_0000);
    UART0.write_str("Hello, world!\nLinea 2\n");
    UART0.write_fmt(format_args!("Running on level EL{}\n", arch::get_el().unwrap() as usize));
}