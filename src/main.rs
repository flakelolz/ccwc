mod args;

use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    /// Size of the file in bytes
    #[arg(short, long)]
    pub count: Option<PathBuf>,

    /// Number of lines in the file
    #[arg(short, long)]
    pub lines: Option<PathBuf>,

    /// Number of words in the file
    #[arg(short, long)]
    pub words: Option<PathBuf>,

    /// Number of characters in the file
    #[arg(short = 'm', long = "chars")]
    pub chars: Option<PathBuf>,

    /// Path to file
    #[arg(short, long)]
    pub path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Byte count
    if let Some(file) = cli.count {
        match file.metadata() {
            Ok(count) => println!("{} bytes", count.len()),
            Err(e) => println!("{}", e),
        }
    }

    // Line count
    if let Some(file) = cli.lines {
        match std::fs::read_to_string(file) {
            Ok(contents) => println!("{} lines", contents.lines().count()),
            Err(e) => println!("{}", e),
        }
    }

    // Word count
    if let Some(file) = cli.words {
        match std::fs::read_to_string(file) {
            Ok(contents) => println!("{} words", contents.split_whitespace().count()),
            Err(e) => println!("{}", e),
        }
    }

    // Character count
    if let Some(file) = cli.chars {
        match std::fs::read_to_string(file) {
            Ok(contents) => println!("{} characters", contents.chars().count()),
            Err(e) => println!("{}", e),
        }
    }
}
