use std::fs::File;
use std::io::{Read, Write};

use crate::todo_item::TodoItem;

pub fn save_to_file(todos: &Vec<TodoItem>) -> std::io::Result<()> {
    let json = serde_json::to_string(&todos).unwrap();
    let mut file = File::create("todos.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn read_from_file() -> std::io::Result<Vec<TodoItem>> {
    let mut file = File::open("todos.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let todos: Vec<TodoItem> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);
    Ok(todos)
}
