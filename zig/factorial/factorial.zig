const std = @import("std");

fn add(a: i32, b: i32) i32 {
    return a + b;
}

fn factorial(n: u32) u32 {
    if (n == 0) return 1;
    return n * factorial(n - 1);
}

fn divide(a: i32, b: i32) ?i32 {
    if (b == 0) return null;
    return @divTrunc(a, b);
}

const Person = struct {
    name: []const u8,
    age: u32,
};

pub fn main() void {
    const result = add(3, 4);
    std.debug.print("3 + 4 = {}\n", .{result});
}
