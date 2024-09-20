use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path
    #[arg(short = 'c')]
    count_bytes_path: Option<PathBuf>,

    #[arg(short = 'l')]
    count_lines_path: Option<PathBuf>,

    #[arg(short = 'w')]
    count_words_path: Option<PathBuf>,

    #[arg(short = 'm')]
    count_characters_path: Option<PathBuf>,

    count_all_path: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file_path = if let Some(ref filePath) = args.count_bytes_path {
        filePath
    } else if let Some(ref filePath) = args.count_lines_path {
        filePath
    } else if let Some(ref filePath) = args.count_words_path {
        filePath
    } else if let Some(ref filePath) = args.count_characters_path {
        filePath
    } else if let Some(ref filePath) = args.count_all_path {
        filePath
    } else {
        panic!("No file path provided");
    };

    let file = File::open(&file_path)?;
    let mut buffered = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    buffered.read_to_end(&mut buffer)?;
    if args.count_bytes_path.is_some() || args.count_all_path.is_some() {
        let byte_count = buffer.len();
        print!("{}  ", byte_count);
    }

    if args.count_characters_path.is_some() {
        // let char_count = buffer.iter().filter(|&&b| b.is_ascii()).count();
        let content = String::from_utf8_lossy(&buffer);
        let char_count = content.chars().count();
        print!("{}  ", char_count);
    }

    if args.count_words_path.is_some() || args.count_all_path.is_some() {
        let content = String::from_utf8_lossy(&buffer);
        let word_count = content.split_whitespace().count();
        print!("{}  ", word_count);
    }

    if args.count_lines_path.is_some() || args.count_all_path.is_some() {
        let content = String::from_utf8_lossy(&buffer);
        let line_count = content.lines().count();
        print!("{}  ", line_count);
    }
    print!("{}\n", file_path.display());

    Ok(())
}
