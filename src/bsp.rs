use core::fmt::Write;

pub mod pl011;

extern "C" {
    pub fn system_off() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn system_off_hook() {
    pl011::UART(0x0900_0000).write_str("--- system off reached ---\n");
}
