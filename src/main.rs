mod entities {
  pub mod todo_item;
}
mod services {
  pub mod commands;
}
mod repository {
  pub mod persistence;
}
mod views {
  pub mod cli;
}

use views::cli::run_cli;

fn main() {
  run_cli();
}
