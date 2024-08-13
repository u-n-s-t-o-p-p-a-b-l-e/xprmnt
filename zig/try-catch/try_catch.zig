const std = @import("std");

const FileError = error {
    NotFound,
    PermissionDenied,
};

fn readFile(path: []const u8) FileError![]const u8 {
    if (std.mem.eql(u8, path, "secret.txt")) {
        return FileError.PermissionDenied;
    }
}
