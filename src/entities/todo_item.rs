use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
  id: u32,
  title: String,
  completed: bool,
}

impl TodoItem {
  pub fn new(id: u32, title: String, completed: bool) -> Self {
      Self { id, title, completed }
  }

  pub fn id(&self) -> u32 {
      self.id
  }

  pub fn title(&self) -> &String {
      &self.title
  }

  pub fn completed(&self) -> bool {
      self.completed
  }

  pub fn set_completed(&mut self, value: bool) {
      self.completed = value;
  }
}
