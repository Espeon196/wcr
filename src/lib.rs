use clap::Parser;

use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;


/// Rust wc
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Arg {
    /// target files
    #[arg(default_values_t = ["-".to_string()])]
    files: Vec<String>,

    /// Show line count
    #[arg(
        short = 'l',
        long = "lines",
    )]
    lines: bool,

    /// Show character count
    #[arg(
        short = 'm',
        long = "chars",
        conflicts_with = "bytes",
    )]
    chars: bool,

    /// Show byte count
    #[arg(
        short = 'c',
        long = "bytes",
        conflicts_with = "chars",
    )]
    bytes: bool,

    /// Show word count
    #[arg(
        short = 'w',
        long = "word",
    )]
    words: bool,
}

impl Arg {
    /// フラグが指定されなかった場合、lines, words, bytesをtrueに設定
    pub fn with_defaults(mut self) -> Self {
        if [self.lines, self.bytes, self.words, self.chars]
            .iter().all(|v| v == &false){
            self.lines = true;
            self.words = true;
            self.bytes = true;
        }
        self
    }
}

pub fn run(args: Arg) -> MyResult<()> {
    println!("{:#?}", args);
    Ok(())
}
