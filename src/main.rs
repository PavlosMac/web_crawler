fn main() {
    if let Err(e) = crawler::run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
