use std::process;

fn main() {
    if let Err(e) = rust_ed::run() {
        println!("Error {}", e);

        process::exit(1);
    }
}
