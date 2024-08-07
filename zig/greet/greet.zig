const std = @import("std");

pub fn main() !void {
    const args = try std.process.argAlloc(std.heap.page_allocator);
    defer std.process.argsFree(std.heap.page_allocator, args);

    std.debug.print("Number of arguments: {d}\n", .{args.len});
    for (args, 0..) |arg, i| {
        std.debug.print("Argument {d}: {s}\n", .{ i, arg });
    }

    const stdout = std.io.getStdOut().writer();
    const stdin = std.io.getStdIn().reader();

    try stdout.print("Enter your name: " .{});
    var buffer: [100]u8 = undefined;
    if (try stdin.readUntilDelimiterOrEof(&buffer, '\n')) |user_input| {
        try stdout.print("Hello, {s}!\n", .{user_input});
    }
}
