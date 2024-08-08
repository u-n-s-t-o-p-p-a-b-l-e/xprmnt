const std = @import("std");

fn add(a: i32, b: i32) i32 {
    return a + b;
}

fn factorial(n: u32) u32 {
    if (n == 0) return 1;
    return n * factorial(n - 1);
}
