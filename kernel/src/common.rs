use super::kernel::putchar;
use core::fmt::{self, Write};

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            putchar(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let mut console = $crate::common::Console;
        let _ = core::fmt::Write::write_fmt(&mut console, core::format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*)  => ($crate::print!("{}\n", core::format_args!($($arg)*)));

}
