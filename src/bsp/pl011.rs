use core::ptr;
use core::fmt;
pub use core::fmt::Write;

pub struct UART (pub u64);

impl UART {
    fn put_char(&mut self, c: u8) {
        let screen = self.0 as *mut u8;

        unsafe {
            ptr::write_volatile(screen, c);
/*
            if 0xA == *screen {
                *screen = 0xD;
                self.wait_tx();
            }
            *screen = c;*/
        }
    }
}

impl fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.put_char(c as u8);
        }
        Ok(())
    }
}