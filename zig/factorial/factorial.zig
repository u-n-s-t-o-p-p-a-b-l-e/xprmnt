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

    const fact5 = factorial(5);
    std.debug.print("Factorial of 5 is {}\n", .{fact5});

    const div_result = divide(10, 2);
    if (div_result) |value| {
        std.debug.print("10 / 2 = {}\n", .{value});
    } else {
        std.debug.print("Cannot divide by zero\n", .{});
    }

    const person = Person {
        .name = "Alice",
        .age = 30,
    };
    std.debug.print("Name: {s}, Age: {}\n", .{ person.name, person.age });
}
