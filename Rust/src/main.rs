use std::collections::HashMap;

use crate::todo::{TodoItem, TodoList, Serialisable};

mod todo;

/*
    argument(args, {
        add: (name: String, synopsis: String, description: String) => {
            todo_list.add_item(name, TodoItem {
                synopsis: synopsis,
                description: description,
                done: false
            });
        },
    })
*/

fn get_appdir() -> String
{
    #[cfg(target_os = "windows")]
    {
        let mut appdir = std::env::var("APPDATA").unwrap();
        appdir.push_str("\\todo");
        return appdir;
    }

    #[cfg(target_os = "linux")]
    {
        let mut appdir = std::env::var("HOME").unwrap();
        appdir.push_str("/.todo");
        return appdir;
    }

    #[cfg(target_os = "macos")]
    {
        let mut appdir = std::env::var("HOME").unwrap();
        appdir.push_str("/Library/Application Support/todo");
        return appdir;
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        panic!("Unsupported operating system");
    }
}

/*
     CLI:

    todo-rs add <title> <synopsis> <description>
    todo-rs remove <title>
    todo-rs complete <title>
    todo-rs uncomplete <title>
    todo-rs list
    todo-rs list done
    todo-rs list undone
    todo-rs list all
    todo-rs list title <title>
*/

fn open_or_create(fname: &str) -> std::fs::File
{
    let appdir = get_appdir();
    std::fs::create_dir_all(appdir.clone()).unwrap();
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(appdir.clone() + "/" + fname).unwrap();
    return file;
}

fn main()
{
    let appdir = get_appdir();
    std::fs::create_dir_all(appdir.clone()).unwrap();
    let mut file = open_or_create("rust.todo");
    let mut todo_list = todo::TodoList::from_file(&mut file);

    let mut args = std::env::args();

    if args.len() == 1 {
        println!("Usage: todo-rs <command> [args]");
        return;
    }

    let command = &args.nth(1).unwrap();
    match command.as_str() {
        "add" => {
            if args.len() != 3 {
                println!("Usage: todo-rs add <title> <synopsis> <description>");
                return;
            }

            let title = args.nth(0).unwrap();
            let synopsis = args.nth(0).unwrap();
            let description = args.nth(0).unwrap();

            todo_list.add_item(title, TodoItem {
                synopsis: synopsis,
                description: description,
                done: false
            });
        },
        "remove" => {
            if args.len() != 1 {
                println!("Usage: todo-rs remove <title>");
                return;
            }

            let title = args.nth(0).unwrap();
            todo_list.remove_item(title);
        },
        "complete" => {
            if args.len() != 1 {
                println!("Usage: todo-rs complete <title>");
                return;
            }

            let title = args.nth(0).unwrap();
            todo_list.complete_item(title);
        },
        "uncomplete" => {
            if args.len() != 1 {
                println!("Usage: todo-rs uncomplete <title>");
                return;
            }

            let title = args.nth(0).unwrap();
            todo_list.uncomplete_item(title);
        },
        "list" => {
            if args.len() == 0 {
                println!("Usage: todo-rs list [done|undone|all|title <title>]");
                return;
            }

            let list_type = &args.nth(0).unwrap();
            match list_type.as_str() {
                "done" => {
                    for (title, item) in todo_list.items.iter() {
                        if item.done {
                            println!("{}: {}", title, item.synopsis);
                        }
                    }
                },
                "undone" => {
                    for (title, item) in todo_list.items.iter() {
                        if !item.done {
                            println!("{}: {}", title, item.synopsis);
                        }
                    }
                },
                "all" => {
                    for (title, item) in todo_list.items.iter() {
                        println!("{}: {}", title, item.synopsis);
                    }
                },
                "title" => {
                    if args.len() != 1 {
                        println!("Usage: todo-rs list title <title>");
                        return;
                    }

                    let title = &args.nth(0).unwrap();
                    let item = todo_list.items.get(title).unwrap();
                    println!("{}: {}", title, item.synopsis);
                    println!("Description: {}", item.description);
                },
                _ => {
                    println!("Usage: todo-rs list [done|undone|all|title <title>]");
                    return;
                }
            }
        },
        "debug" => {
            println!("{:?}", todo_list);
        },
        _ => {
            println!("Usage: todo-rs <command> [args]");
            return;
        }
    };
    todo_list.save(&mut file);
}
