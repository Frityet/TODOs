const std = @import("std");
const todo = @import("./todo.zig");

pub fn main() !void
{
    var todolist = todo.TodoList.create("todo.txt", "test");
    std.debug.print("{}", .{todolist});
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Hello, world!", .{});
}

