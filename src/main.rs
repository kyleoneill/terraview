use std::env;
use std::process;
mod lib;

fn main() {
    let _args = env::args();
    if let Err(e) = lib::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}