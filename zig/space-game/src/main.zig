const std = @import("std");
const rl = @import("raylib");

pub fn main() !void {
    std.debug.print("Hi There!\n", .{});
    rl.initWindow(1200, 720, "LARGE SPACE ROCKS");
    rl.setTargetFPS(60);

    while (!rl.windowShouldClose()) {
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(rl.Color.white);
    }
}
