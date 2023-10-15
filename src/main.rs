mod entities;
mod repository;
mod services;
mod views;

use views::cli::run_cli;
use views::gui::run_gui;

pub fn main() {
  if true {
    run_gui();
  } else {
    run_cli();
  }
}
