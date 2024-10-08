const std = @import("std");

const FileError = error {
    NotFound,
    PermissionDenied,
};

fn readFile(path: []const u8) FileError![]const u8 {
    if (std.mem.eql(u8, path, "secret.txt")) {
        return FileError.PermissionDenied;
    }
    if (std.mem.eql(u8, path, "missing.txt")) {
        return FileError.NotFound;
    }
    return "File contents";
}

fn Stack(comptime T: type) type {
    return struct {
        items: std.ArrayList(T),

        const Self = @This();

        pub fn init(allocator: std.mem.Allocator) Self {
            return Self {
                .items = std.ArrayList(T).init(allocator),
            };
        }

        pub fn push(self: *Self, value: T) !void {
            try self.items.append(value);
        }

        pub fn pop(self: *Self) ?T {
            return if (self.items.items.len == 0) null else self.items.pop();
        }

        pub fn deinit(self: *Self) void {
            self.items.deinit();
        }
    };
}

pub fn main() !void {
    const file_result = readFile("secret.txt") catch |err| switch (err) {
        FileError.NotFound => "File not found",
        FileError.PermissionDenied => "Access denied",
    };
    std.debug.print("File read result: {s}\n", .{file_result});

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var intStack = Stack(i32).init(allocator);
    defer intStack.deinit();

    try intStack.push(10);
    try intStack.push(20);
    try intStack.push(30);

    while (intStack.pop()) |value| {
        std.debug.print("Popped: {}\n", .{value});
    }

    const computed_array = comptime blk: {
        var arr: [5]i32 = undefined;
        for (&arr, 0..) |*item, i| {
            item.* = @as(i32, @intCast(i)) * @as(i32, @intCast(i));
        }
        break :blk arr;
    };
    std.debug.print("Computed array: {any}\n", .{computed_array});
}
