use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args = env::args();

    let config: Config =  Config::new(args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(err) = minigrep::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    };
}