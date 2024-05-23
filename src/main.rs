use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Size of the file in bytes
    #[arg(short, long, action)]
    pub count: bool,

    /// Number of lines in the file
    #[arg(short, long, action)]
    pub lines: bool,

    /// Number of words in the file
    #[arg(short, long, action)]
    pub words: bool,

    /// Number of characters in the file
    #[arg(short = 'm', long = "chars", action)]
    pub chars: bool,

    /// Path to file
    pub path: PathBuf,
}

fn main() {
    let args = Args::parse();

    match std::fs::read_to_string(&args.path) {
        Ok(contents) => {
            if !args.count && !args.lines && !args.words && !args.chars {
                print!("{} ", contents.lines().count());
                print!("{} ", contents.split_whitespace().count());
                print!("{} ", contents.len());
            }

            if args.count {
                print!("{} ", contents.len());
            }
            if args.lines {
                print!("{} ", contents.lines().count());
            }
            if args.words {
                print!("{} ", contents.split_whitespace().count());
            }
            if args.chars {
                print!("{} ", contents.chars().count());
            }

            println!("{} ", args.path.display());
        }
        Err(e) => println!("{}", e),
    }
}
