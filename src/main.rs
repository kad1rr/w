mod git;
mod colors;

use std::env;
use colors::Colors;
use git::get_branch;

fn main() {
    print!("\n");
    let branch = match get_branch() {
        Ok(b) => b + "  ",
        Err(_) => "None".to_string(),
    };

    let symbol = "→".to_string();

    let dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    if branch != "None" {
        println!("{} {}{} {}{}", symbol, Colors::yellow(""), Colors::yellow_bg(&dir) + &*Colors::yellow(&""), Colors::blue(""), Colors::blue_bg(&branch) + &*Colors::blue(&""));
    } else {
        println!("{} {}{}", symbol, Colors::yellow(""), Colors::yellow_bg(&dir) + &*Colors::yellow(&""));
    }

    print!("\n");
}
