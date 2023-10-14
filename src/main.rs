struct TodoItem {
  id: u32,
  title: String,
  completed: bool,
}

fn add_todo(todos: &mut Vec<TodoItem>, title: String) {
  let new_id = todos.len() as u32 + 1;
  let new_todo = TodoItem {
      id: new_id,
      title,
      completed: false,
  };
  todos.push(new_todo);
}

fn complete_todo(todos: &mut Vec<TodoItem>, id: u32) {
  if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
      todo.completed = true;
  }
}

fn list_todos(todos: &Vec<TodoItem>) {
  for todo in todos {
      println!("ID: {}, Title: {}, Completed: {}", todo.id, todo.title, todo.completed);
  }
}

fn main() {
  let mut todos: Vec<TodoItem> = Vec::new();
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
}
