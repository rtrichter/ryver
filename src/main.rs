fn main() {
    if let Err(error) = ryver::cli::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
