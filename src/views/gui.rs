use crate::entities::todo_item::TodoItem;
use crate::repository::persistence::{read_from_file, save_to_file};
use crate::services::commands::{complete_todo, uncomplete_todo};
use iced::{
    Application, Command, Element, Length, Settings, Theme, 
    widget::{Column, Checkbox}, 
    executor::Default as DefaultExecutor,
};

#[derive(Debug, Clone)]
enum Message {
    CheckboxToggled(usize, bool),
}

struct TodoApp {
    todos: Vec<TodoItem>,
}

impl Application for TodoApp {
    type Executor = DefaultExecutor;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                todos: read_from_file().unwrap_or_else(|_| vec![]),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Todo App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CheckboxToggled(index, value) => {
                if let Some(todo) = self.todos.get_mut(index) {
                    todo.completed = value;
                    let id: u32 = index as u32;
                    if value {
                        complete_todo(&mut self.todos, id);
                    } else {
                        uncomplete_todo(&mut self.todos, id);
                    }
                    if let Err(e) = save_to_file(&self.todos) {
                        println!("Failed to save todos: {}", e);
                    }
                } else {
                    println!("Invalid index {} provided for CheckboxToggled", index);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut column = Column::new();
        for (index, todo) in self.todos.iter().enumerate() {
            column = column.push(Checkbox::new(
                todo.title.clone(),
                todo.completed,
                move |value| Message::CheckboxToggled(index, value),
            ));
        }
        column.width(Length::Fill).height(Length::Fill).into()
    }
}

pub fn run_gui() {
    TodoApp::run(Settings::default());
}
