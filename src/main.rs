use std::process;

fn main() {
    if let Err(err) = rqst::parse_args().and_then(rqst::run) {
        eprintln!("rqst: {}", err);
        process::exit(1);
    }
}
