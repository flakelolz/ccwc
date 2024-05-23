use clap::Parser;
use std::{
    io::Read,
    path::{Path, PathBuf},
};

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
    pub path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // check if a path argument was provided
    match &args.path {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(contents) => {
                cw(&args, contents, Some(path));
            }
            Err(e) => println!("{}", e),
        },
        // if not, check stdin for contents
        None => {
            let mut contents = String::new();
            match std::io::stdin().read_to_string(&mut contents) {
                Ok(_) => {
                    cw(&args, contents, None);
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}

fn cw(args: &Args, contents: String, path: Option<&Path>) {
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

    match path {
        Some(path) => println!("{} ", path.display()),
        None => println!(),
    }
}
