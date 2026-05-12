fn main() {
    if let Err(e) = claude_profile::cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
