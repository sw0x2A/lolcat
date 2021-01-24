use std::process;

use lolcat::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lolcat::run(&config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
