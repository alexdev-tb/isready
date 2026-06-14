mod checks;
mod config;

use colored::Colorize;
use config::*;
fn main() {
    let contents = std::fs::read_to_string("isready.toml").unwrap();
    let parsed: Config = toml::from_str(&contents).unwrap();
    println!(
        "{} {}\n",
        "Running checks for:".bold(),
        parsed.project.name.bright_white()
    );
    for check in parsed.check {
        print!("{}:  ", check.name().bold());
        let run = check.run();
        if run {
            println!("{}", "passed".green())
        } else {
            println!("{}", "failed".red())
        }
    }
}
