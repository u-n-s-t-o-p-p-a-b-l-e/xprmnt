#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern "C" {
    fn getpid() -> i32;
    fn write(fd: i32, buf: *const u8, count: usize) -> isize;
}
