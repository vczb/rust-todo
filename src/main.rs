use std::env;

mod entities;
mod repository;
mod services;
mod views;

use views::cli::run_cli;
use views::gui::run_gui;

pub fn main() {
  let args: Vec<String> = env::args().collect();

  if args.contains(&"--gui".to_string()) {
    run_gui();
  } else {
    run_cli();
  }
}
