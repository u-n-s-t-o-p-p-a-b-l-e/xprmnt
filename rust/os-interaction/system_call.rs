#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern "C" {
    fn getpid() -> i32;
    fn write(fd: i32, buf: *const u8, count: usize) -> isize;
}

fn main() {
    unsafe {
        let pid = getpid();
        println!("Process ID: {}", pid);

        let message = b"Hello from Rust!\n";
        write(1, message.as_ptr(), message.len());
    }
}
