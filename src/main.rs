use std::env;
use std::process;
mod lib;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Need to supply a world name.");
    args.remove(0);
    let world_name = args.join(" ");
    if let Err(e) = lib::run(world_name) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}