const std = @import("std");
const ArrayList = std.ArrayList;
const print = std.debug.print;

const Todo = struct {
    id: usize,
    description: []u8,
    completed: bool,
};

const FILENAME = "todos.txt";

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const leaked = gpa.deinit();
        if (leaked == .leak) std.debug.print("Error: memory leak detected\n", .{});
    }
    const allocator = gpa.allocator();

    var todos = ArrayList(Todo).init(allocator);
    defer {
        for (todos.items) |todo| {
            allocator.free(todo.description);
        }
        todos.deinit();
    }

    try loadTodos(&todos, allocator);

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    const command = args[1];
    if (std.mem.eql(u8, command, "add")) {
        if (args.len < 3) {
            print("Error: 'add' requires a description\n", .{});
            return;
        }
        try addTodo(&todos, allocator, args[2]);
    } else if (std.mem.eql(u8, command, "list")) {
        listTodos(todos);
    } else if (std.mem.eql(u8, command, "complete")) {
        if (args.len < 3) {
            print("Error: 'complete' requires a todo ID\n", .{});
            return;
        }
        const id = try std.fmt.parseInt(usize, args[2], 10);
        try completeTodo(&todos, id);
    } else {
        print("Unknown command: {s}\n", .{command});
        printUsage();
    }

    try saveTodos(todos);
}

fn printUsage() void {
    print("Usage:\n", .{});
    print("  add <description>    Add a new todo\n", .{});
    print("  list                 List all todos\n", .{});
    print("  complete <id>        Mark a todo as completed\n", .{});
}

fn addTodo(todos: *ArrayList(Todo), allocator: std.mem.Allocator, description: []const u8) !void {
    const desc = try allocator.dupe(u8, description);
    errdefer allocator.free(desc);

    const todo = Todo{
        .id = todos.items.len + 1,
        .description = desc,
        .completed = false,
    };
    try todos.append(todo);
    print("Added todo: {d}. {s}\n", .{ todo.id, todo.description });
}

fn listTodos(todos: ArrayList(Todo)) void {
    for (todos.items) |todo| {
        const status = if (todo.completed) "✓" else " ";
        print("[{s}] {d}. {s}\n", .{ status, todo.id, todo.description });
    }
}

fn completeTodo(todos: *ArrayList(Todo), id: usize) !void {
    for (todos.items) |*todo| {
        if (todo.id == id) {
            todo.completed = true;
            print("Marked todo {d} as completed\n", .{id});
            return;
        }
    }
    print("Todo with id {d} not found\n", .{id});
}

fn saveTodos(todos: ArrayList(Todo)) !void {
    const file = try std.fs.cwd().createFile(FILENAME, .{});
    defer file.close();

    const writer = file.writer();
    for (todos.items) |todo| {
        try writer.print("{d}|{s}|{}\n", .{ todo.id, todo.description, todo.completed });
    }
}

fn loadTodos(todos: *ArrayList(Todo), allocator: std.mem.Allocator) !void {
    const file = std.fs.cwd().openFile(FILENAME, .{}) catch |err| switch (err) {
        error.FileNotFound => return,
        else => return err,
    };
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var iter = std.mem.split(u8, line, "|");
        const id = try std.fmt.parseInt(usize, iter.next().?, 10);
        const description = try allocator.dupe(u8, iter.next().?);
        errdefer allocator.free(description);
        const completed = if (std.mem.eql(u8, iter.next().?, "true")) true else false;

        try todos.append(Todo{ .id = id, .description = description, .completed = completed });
    }
}
