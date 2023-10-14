use crate::commands::{add_todo, complete_todo, list_todos, remove_todo};
use crate::persistence::{save_to_file, read_from_file};

pub fn run_cli() {

  let mut todos = read_from_file().unwrap_or_else(|_| vec![]);

  loop {
      println!("What would you like to do?");
      println!("1: Add Todo");
      println!("2: Complete Todo");
      println!("3: List Todos");
      println!("4: Remove Todo");
      println!("5: Quit");

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
          "4" => {
            println!("Enter the ID of the todo to mark as completed:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id).unwrap();
            let id: u32 = id.trim().parse().unwrap();
            remove_todo(&mut todos, id);
          },
          "5" => break,
          _ => println!("Invalid choice, please try again."),
      }
  }

  if let Err(err) = save_to_file(&todos) {
    eprintln!("Error saving todos: {}", err);
  }
}