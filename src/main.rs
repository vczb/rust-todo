mod todo_item;
mod commands;
mod persistence;
mod cli;

use cli::run_cli;

fn main() {
  run_cli(); 
}
