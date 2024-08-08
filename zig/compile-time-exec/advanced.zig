const std = @import("std");

const MyStruct = struct {
    value:  i32,

    pub fn init(value: i32) MyStruct {
        return MyStruct{ .value = value };
    }

    pub fn display(self: MyStruct) void {
        std.debug.print("Value: {}\n", .{self.value});
    }
};

pub fn main() !void {
    var allocator = std.heap.page_allocator;
    const instance = try createMyStruct(&allocator, 42);

    defer destroyMyStruct(&allocator, instance);

    instance.display();
    comptimeExample();
}
