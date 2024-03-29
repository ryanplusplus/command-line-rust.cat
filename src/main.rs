fn main() {
    if let Err(e) = cat::get_args().and_then(cat::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
