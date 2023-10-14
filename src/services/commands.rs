use crate::entities::todo_item::TodoItem;

pub fn add_todo(todos: &mut Vec<TodoItem>, title: String) {
    let new_id = todos.len() as u32 + 1;
    let new_todo = TodoItem::new(new_id, title, false);
    todos.push(new_todo);
}

pub fn remove_todo(todos: &mut Vec<TodoItem>, id: u32) {
  todos.retain(|todo| todo.id() != id);
}

pub fn complete_todo(todos: &mut Vec<TodoItem>, id: u32) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id() == id) {
        todo.set_completed(true);
    }
}

pub fn list_todos(todos: &Vec<TodoItem>) {
    for todo in todos {
        println!("ID: {}, Title: {}, Completed: {}", todo.id(), todo.title(), todo.completed());
    }
}
