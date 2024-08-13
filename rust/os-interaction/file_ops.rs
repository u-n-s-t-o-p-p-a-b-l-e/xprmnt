#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern "C" {
    fn open(pathname: *const u8, flags: i32, mode: i32) -> i32;
    fn write(fd: i32, buf: *const 8, count: usize) -> isize;
    fn close(fd: i32) -> i32;
}

const O_WRONLY: i32 = 1;
const O_CREAT: i32 = 64;
const S_IRUSR: i32 = 0o400;
const S_IWUSR: i32 = 0o200;

fn main() {
    let filename = b"example.txt\0";
    let message = b"Hello, world!\n";

    unsafe {
        let fd = open(filename.as_ptr(), O_WRONLY | O_CREAT, S_IRUSR | S_IWUSR);
        if fd < 0 {
            eprintln!("Failed to open the file");
            return;
        }

        let bytes_written = write(fd, message.as_ptr(), message.len());
        if bytes_written < 0 {
            eprintln!("Failed to write to the file");
            close(fd);
            return;
        }

        if close(fd) < 0 {
            eprintln!("Failed to close the file");
        } else {
            println!("File operation completed successfully.");
        }
    }
}
