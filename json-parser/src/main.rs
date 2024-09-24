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
    let mut stack = Vec::<char>::new();
    let mut is_valid_json = false;
    let mut key_count = 0;
    let mut key_name = String::new();
    let mut map = std::collections::HashMap::new();

    for val in args.object.chars() {
        //println!("val: {}", val);
        if val == '}' && stack.last().unwrap() == &'{' && key_count == map.len() {
            is_valid_json = true;
            break;
        } else if val == ' ' {
            continue;
        } else if val == '"' && stack.last().unwrap().is_alphanumeric() {
            let mut name = String::new();
            while stack.last().unwrap() != &'"' {
                name += stack.pop().unwrap().to_string().as_str();
            }
            if key_name.is_empty() {
                key_name = name;
            } else {
                map.insert(key_name, name);
                key_name = String::new();
            }
            stack.pop();
            continue;
        } else if val == ':' {
            key_count += 1;
            println!("key_count: {}", key_count);
            continue;
        }
        stack.push(val);
    }
    println!("map: {:?}", map);
    println!("stack: {:?}", stack);

    println!("Is valid json: {}", is_valid_json);
}

// Tests - Read tests folder iterate and run all Tests
