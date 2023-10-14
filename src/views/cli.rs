use crate::services::commands::{add_todo, complete_todo, list_todos, remove_todo, uncomplete_todo};
use crate::repository::persistence::{save_to_file, read_from_file};

pub fn run_cli() {

  let mut todos = read_from_file().unwrap_or_else(|_| vec![]);

  loop {
    println!("What would you like to do?");
    println!("0: Quit");
    println!("1: Add Todo");
    println!("2: List Todos");
    println!("3: Complete Todo");
    println!("4: Uncomplete Todo");
    println!("5: Remove Todo");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    match choice.trim().as_ref() {
        "0" => break,
        "1" => {
            println!("Enter the title of your new todo:");
            let mut title = String::new();
            std::io::stdin().read_line(&mut title).unwrap();
            add_todo(&mut todos, title.trim().to_string());
        },
        "2" => {
            list_todos(&todos);
        },
        "3" => {
            println!("Enter the ID of the todo to mark as completed:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id).unwrap();
            let id: u32 = id.trim().parse().unwrap();
            complete_todo(&mut todos, id);
        },
        "4" => {
            println!("Enter the ID of the todo to mark as uncompleted:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id).unwrap();
            let id: u32 = id.trim().parse().unwrap();
            uncomplete_todo(&mut todos, id);
        },
        "5" => {
            println!("Enter the ID of the todo to remove:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id).unwrap();
            let id: u32 = id.trim().parse().unwrap();
            remove_todo(&mut todos, id);
        },
        _ => println!("Invalid choice, please try again."),
    }
  }


  if let Err(err) = save_to_file(&todos) {
    eprintln!("Error saving todos: {}", err);
  }
}