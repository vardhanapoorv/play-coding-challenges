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
    let mut key_str = String::new();
    // let mut map = std::collections::HashMap::new();
    let mut keys = Vec::<String>::new();

    for val in args.object.chars() {
        //println!("val: {}", val);
        if val == '}' && stack.last().unwrap() == &'{' {
            if key_str.len() > 0 {
                keys.push(key_str);
                key_str = String::new();
            }
            is_valid_json = true;
            break;
        } else if val == ' ' || val == '\n' || val == '\t' {
            continue;
        } else if val == '"' && *stack.last().unwrap() == '"' {
            stack.pop();
            println!("stack {:?}", stack);
            continue;
        } else if val == '"' {
            if *stack.last().unwrap() == ':' || *stack.last().unwrap() == ',' {
                stack.pop();
            }
            stack.push(val);
            continue;
        } else if !stack.is_empty() && *stack.last().unwrap() == '"' {
            key_str += val.to_string().as_str();
            println!("key_str {:?}", key_str);
            continue;
        } else if val == ':' {
            println!("key_str {:?}", key_str);
            if key_str.len() > 0 {
                keys.push(key_str);
                key_str = String::new();
            }
            stack.push(val);
            continue;
        } else if val == ',' {
            if key_str.len() > 0 {
                keys.push(key_str);
                key_str = String::new();
            }
            stack.push(val);
            continue;
        } else if val.is_alphanumeric() {
            is_valid_json = false;
            break;
        }
        stack.push(val);
    }

    if keys.len() > 0 {
        // map.insert(keys[0].clone(), keys[1].clone());
    }
    // map.insert(keys[0].clone(), keys[1].clone());
    // map.insert("d", "d");
    // println!("map: {:?}", map);
    println!("stack: {:?}", stack);
    println!("keys: {:?}", keys);

    println!("Is valid json: {}", is_valid_json);
}

// Tests - Read tests folder iterate and run all Tests
