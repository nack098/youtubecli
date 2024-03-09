use std::env;
use youtubecli::run;
use colored::Colorize;

fn main() -> () {
    match run(&env::args().collect()) {
        Ok(_) => {},
        Err(e) => {
            println!("
                {:^3}\n
                {}",
                e[0].red(),
                e[1].blue())
        }
    };
}