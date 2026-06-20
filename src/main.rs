mod checks;
mod config;

use colored::Colorize;
use config::*;
use std::{
    fs,
    io::{self, Write},
    process::exit,
};
fn main() {
    if !fs::exists("isready.toml").unwrap() {
        println!("{}", "Config file not found!".red());
        exit(2);
    }
    let contents = match std::fs::read_to_string("isready.toml") {
        Ok(c) => c,
        Err(e) => {
            println!("{} {}", "Failed to read isready.toml".red(), e);
            exit(2)
        }
    };

    let parsed: Config = match toml::from_str(&contents) {
        Ok(p) => p,
        Err(e) => {
            println!("{} {}", "Failed to parse isready.toml".red(), e);
            exit(2)
        }
    };
    println!(
        "{} {}\n",
        "Running checks for:".bold(),
        parsed.project.name.bright_white()
    );
    let mut allpassed = true;
    for check in parsed.check {
        print!("{}:  ", check.name().bold());
        io::stdout().flush().unwrap();
        let run = check.run();
        if run {
            println!("{}", "passed".green())
        } else {
            println!("{}", "failed".red());
            allpassed = false
        }
    }
    exit(if allpassed { 0 } else { 1 })
}
