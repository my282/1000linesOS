use super::kernel::putchar;
use core::fmt::{self, Write};
use core::ptr;

pub struct Console;
pub const PAGE_SIZE: u32 = 4096;

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

pub unsafe fn memcpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut d = dst;
    let mut s = src;
    for _ in 0..n {
        unsafe {
            ptr::write_volatile(d, *s);
            d = d.add(1);
            s = s.add(1);
        }
    }
}

pub unsafe fn memset(buf: *mut u8, c: u8, n: usize) {
    let mut p = buf;
    for _ in 0..n {
        unsafe {
            ptr::write_volatile(p, c);
            p = p.add(1);
        }
    }
}

pub unsafe fn strcpy(dst: *mut u8, src: &str) {
    let mut d = dst;
    for c in src.bytes() {
        unsafe {
            ptr::write_volatile(d, c);
            d = d.add(1);
        }
    }
    unsafe {
        ptr::write_volatile(d, 0);
    }
}

pub unsafe fn strcmp(s1: *const u8, s2: *const u8) -> isize {
    let mut s1 = s1;
    let mut s2 = s2;
    unsafe {
        while *s1 != 0 && *s2 != 0 {
            if *s1 != *s2 {
                break;
            }
            s1 = s1.add(1);
            s2 = s2.add(1);
        }

        *s1 as isize - *s2 as isize
    }
}
