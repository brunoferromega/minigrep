use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Applicaton error: {e}");
        process::exit(1);
    }
}

