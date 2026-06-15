mod checks;
mod config;

use colored::Colorize;
use config::*;
use std::{
    fs,
    io::{self, Write},
};
fn main() {
    if !fs::exists("isready.toml").unwrap() {
        println!("{}", "Config file not found!".red());
        return;
    }
    let contents = std::fs::read_to_string("isready.toml").unwrap();
    let parsed: Config = toml::from_str(&contents).unwrap();
    println!(
        "{} {}\n",
        "Running checks for:".bold(),
        parsed.project.name.bright_white()
    );
    for check in parsed.check {
        print!("{}:  ", check.name().bold());
        io::stdout().flush().unwrap();
        let run = check.run();
        if run {
            println!("{}", "passed".green())
        } else {
            println!("{}", "failed".red())
        }
    }
}
