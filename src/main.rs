use std::process::exit;

use cli::app;

mod cli;
mod interpreter;
mod parser;

fn main() {
    match app() {
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
        _ => (),
    }
}
