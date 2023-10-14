mod todo_item;
mod commands;

use crate::todo_item::TodoItem;
use crate::commands::{add_todo, complete_todo, list_todos};

use std::fs::File;
use std::io::Write;
use std::io::Read;

pub fn read_from_file() -> std::io::Result<Vec<TodoItem>> {
    let mut file = File::open("todos.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let todos: Vec<TodoItem> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);
    Ok(todos)
}

pub fn save_to_file(todos: &Vec<TodoItem>) -> std::io::Result<()> {
    let json = serde_json::to_string(&todos).unwrap();
    let mut file = File::create("todos.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn main() {
  
  let mut todos = read_from_file().unwrap_or_else(|_| vec![]);

  loop {
      println!("What would you like to do?");
      println!("1: Add Todo");
      println!("2: Complete Todo");
      println!("3: List Todos");
      println!("4: Quit");

      let mut choice = String::new();
      std::io::stdin().read_line(&mut choice).unwrap();

      match choice.trim().as_ref() {
          "1" => {
              println!("Enter the title of your new todo:");
              let mut title = String::new();
              std::io::stdin().read_line(&mut title).unwrap();
              add_todo(&mut todos, title.trim().to_string());
          },
          "2" => {
              println!("Enter the ID of the todo to mark as completed:");
              let mut id = String::new();
              std::io::stdin().read_line(&mut id).unwrap();
              let id: u32 = id.trim().parse().unwrap();
              complete_todo(&mut todos, id);
          },
          "3" => {
              list_todos(&todos);
          },
          "4" => break,
          _ => println!("Invalid choice, please try again."),
      }
  }

  if let Err(err) = save_to_file(&todos) {
    eprintln!("Error saving todos: {}", err);
  }
}
