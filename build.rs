fn main() {
    if let Err(error) = built::write_built_file() {
        eprintln!("failed to gather build-time information: {error}");
        std::process::exit(1);
    }
}
