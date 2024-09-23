// cli using clap

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    object: String,
}

fn main() {
    let args = Args::parse();
    let mut stack = Vec::new();
    let mut is_valid_json = false;

    for val in args.object.chars() {
        if val == '}' && stack.last().unwrap() == &'{' {
            is_valid_json = true;
            break;
        }
        stack.push(val);
    }

    println!("Is valid json: {}", is_valid_json);
}
