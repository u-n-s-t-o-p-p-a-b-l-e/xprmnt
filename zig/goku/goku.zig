const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var lookup = std.StringHashMap(User).init(allocator);
    defer lookup.deinit();

    const goku = User{ .power = 9001 };

    try lookup.put("Goku", goku);

    const entry = lookup.getPtr("Goku").?;

    std.debug.print("Goku's power is: {d}\n", .{entry.power});

    _ = lookup.remove("Goku");

    std.debug.print("Goku's power is: {d}\n", .{entry.power});
}

const User = struct {
    power: i32,
}
