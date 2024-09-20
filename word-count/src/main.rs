use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'c')]
    count_bytes: bool,

    #[arg(short = 'l')]
    count_lines: bool,

    #[arg(short = 'w')]
    count_words: bool,

    #[arg(short = 'm')]
    count_characters: bool,

    file_path: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let input: Box<dyn Read> = if let Some(file_path) = args.file_path.as_ref() {
        Box::new(File::open(file_path)?)
    } else {
        println!("No file path provided, reading from stdin");
        Box::new(io::stdin())
    };

    let mut buffered = BufReader::new(input);
    let mut buffer: Vec<u8> = Vec::new();
    buffered.read_to_end(&mut buffer)?;

    let count_all = !(args.count_bytes || args.count_words || args.count_lines);

    if args.count_bytes || count_all {
        let byte_count = buffer.len();
        print!("{}  ", byte_count);
    }

    let content = String::from_utf8_lossy(&buffer);

    if args.count_characters {
        let char_count = content.chars().count();
        print!("{}  ", char_count);
    }

    if args.count_words || count_all {
        let word_count = content.split_whitespace().count();
        print!("{}  ", word_count);
    }

    if args.count_lines || count_all {
        let line_count = content.lines().count();
        print!("{}  ", line_count);
    }

    println!();

    Ok(())
}
