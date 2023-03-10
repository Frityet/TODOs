const std = @import("std");
const io = std.io;
const HashMap = std.hash_map.HashMap;
const String = std.string.String;

pub const TodoItem = struct {
    title: String,
    description: String,
    done: bool,
};

pub const TodoList = struct {
    title: String,
    description: String,
    items: HashMap(String, TodoItem),

    pub fn create(title: String, description: String) TodoList {
        return TodoList {
            .title = title,
            .description = description,
            .items = HashMap(String, TodoItem).init(),
        };
    }
};

