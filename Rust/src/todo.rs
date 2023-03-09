// Copyright (C) 2023 Amrit Bhogal
//
// This file is part of Rust.
//
// Rust is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rust is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Rust.  If not, see <http://www.gnu.org/licenses/>.

use std::{collections::HashMap, io::{Write, Read, Seek}};

pub trait Serialisable {
    fn serialise(&self) -> Box<[u8]>;
    fn deserialise(data: &[u8]) -> Self;
}

#[derive(Debug)]
pub struct TodoItem {
    pub synopsis: String,
    pub description: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct TodoList {
    pub title: String,
    pub description: String,
    pub items: HashMap<String, TodoItem>,
}

impl TodoList {
    pub fn create(title: String) -> TodoList
    {
        let mut file = std::fs::File::create(title.clone()).unwrap();
        let mut todo_list = TodoList {
            title: title.clone(),
            description: String::new(),
            items: HashMap::new(),
        };

        return todo_list;
    }

    pub fn from_file(file: &mut std::fs::File) -> TodoList
    {
        //if the file is empty, create a new todo list
        if file.metadata().unwrap().len() == 0 {
            return TodoList::create("rust.todo".to_string());
        }
        //get all bytes
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        return TodoList::deserialise(data.as_ref());
    }

    pub fn add_item(&mut self, title: String, item: TodoItem)
    {
        self.items.insert(title, item);
    }

    pub fn remove_item(&mut self, title: String)
    {
        self.items.remove(&title);
    }

    pub fn get_item(&mut self, title: String) -> Option<&mut TodoItem>
    {
        return self.items.get_mut(&title);
    }

    pub fn complete_item(&mut self, title: String)
    {
        if let Some(item) = self.get_item(title) {
            item.done = true;
        }
    }

    pub fn uncomplete_item(&mut self, title: String)
    {
        if let Some(item) = self.get_item(title) {
            item.done = false;
        }
    }

    pub fn save(&mut self, file: &mut std::fs::File)
    {
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        file.write_all(self.serialise().as_ref()).unwrap();
    }
}

impl Serialisable for TodoList {
    fn serialise(&self) -> Box<[u8]>
    {
        let mut data = Vec::new();
        data.push(self.title.len() as u8);
        data.extend(self.title.as_bytes());
        data.push(self.description.len() as u8);
        data.extend(self.description.as_bytes());
        data.push(self.items.len() as u8);
        for (title, item) in self.items.iter() {
            data.push(title.len() as u8);
            data.extend(title.as_bytes());
            data.push(item.synopsis.len() as u8);
            data.extend(item.synopsis.as_bytes());
            data.push(item.description.len() as u8);
            data.extend(item.description.as_bytes());
            data.push(item.done as u8);
        }
        return data.into_boxed_slice();
    }

    fn deserialise(data: &[u8]) -> Self
    {
        let mut title = String::new();
        let mut description = String::new();
        let mut items = HashMap::new();
        let mut i = 0;
        let title_len = data[i] as usize;
        i += 1;
        title.extend(data[i..i + title_len].iter().map(|&x| x as char));
        i += title_len;
        let description_len = data[i] as usize;
        i += 1;
        description.extend(data[i..i + description_len].iter().map(|&x| x as char));
        i += description_len;
        let items_len = data[i] as usize;
        i += 1;
        for _ in 0..items_len {
            let title_len = data[i] as usize;
            i += 1;
            let mut title = String::new();
            title.extend(data[i..i + title_len].iter().map(|&x| x as char));
            i += title_len;
            let synopsis_len = data[i] as usize;
            i += 1;
            let mut synopsis = String::new();
            synopsis.extend(data[i..i + synopsis_len].iter().map(|&x| x as char));
            i += synopsis_len;
            let description_len = data[i] as usize;
            i += 1;
            let mut description = String::new();
            description.extend(data[i..i + description_len].iter().map(|&x| x as char));
            i += description_len;
            let done = data[i] != 0;
            i += 1;
            items.insert(title, TodoItem {
                synopsis: synopsis,
                description: description,
                done: done,
            });
        }
        return TodoList {
            title: title,
            description: description,
            items: items,
        };
    }
}
