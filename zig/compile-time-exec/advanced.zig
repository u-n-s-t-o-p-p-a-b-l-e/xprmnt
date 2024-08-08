const std = @import("std");

const MyStruct = struct {
    value:  i32,

    pub fn init(value: i32) MyStruct {
        return MyStruct{ .value = value };
    }

    pub fn display(self: MyStruct) void {
        std.debug.print("Value: {}\n", .{self.value});
    }
}
