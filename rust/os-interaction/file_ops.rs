#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern "C" {
    fn open(pathname: *const u8, flags: i32, mode: i32) -> i32;
    fn write(fd: i32, buf: *const 8, count: usize) -> isize;
    fn close(fd: i32) -> i32;
}
