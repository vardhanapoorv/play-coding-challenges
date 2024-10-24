use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'f')]
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

    println!("Read {} bytes", buffer.len());

    Ok(())
}
