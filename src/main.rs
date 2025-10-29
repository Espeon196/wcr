use clap::Parser;

fn main() {
    let matches = wcr::Arg::parse().with_defaults();

    if let Err(err) = wcr::run(matches) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
