use std::process;

fn main() {
    if let Err(e) = game_of_life::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
