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
    println!("Hello, world!");
}
