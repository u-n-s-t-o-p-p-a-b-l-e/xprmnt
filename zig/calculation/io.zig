const std = @import("std");

fn add(a: f64, b: f64) f64 {
    return a + b;
}

fn subtract(a: f64, b: f64) f64 {
    return a - b;
}

fn multiply(a: f64, b: f64) f64 {
    return a * b;
}

fn divide(a: f64, b: f64) f64 {
    if(b == 0) {
        return error.DivisionByZero;
    }
    return a / b;
}

pub fn main() !void {
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();

    const op = try.getValidOperation(stdin, stdout);
    const num1 = try getValidFloat(stdin, stdout, "Enter first number: ");
    const num2 = try getValidFloat(stdin, stdout, "Enter second number: ");

    var result: f64 = undefined;
    switch (op) {
        '+' => result = add(num1, num2),
        '-' => result = subtract(num1, num2),
        '*' => result = multiply(num1, num2),
        '/' => result = try divide(num1, num2),
        else => unreachable,
    }

    try  stdout.print("Result: {d}\n", .{result});
}
