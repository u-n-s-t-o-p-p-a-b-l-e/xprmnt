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

    var gpa = std.heap.generalPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
}
