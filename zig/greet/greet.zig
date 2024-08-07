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
}
