// cli using clap
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    object: String,
}

struct SpecialChars;

impl SpecialChars {
    pub const OPEN_BRACE: char = '{';
    pub const CLOSE_BRACE: char = '}';
    pub const COMMA: char = ',';
    pub const COLON: char = ':';
    pub const QUOTE: char = '"';
}

enum JsonValueType {
    STRING,
    NUMBER,
    OBJECT,
    ARRAY,
    BOOLEAN,
    NULL,
}

fn main() {
    let args = Args::parse();
    let mut stack = Vec::<char>::new();
    let mut stack_new = Vec::<char>::new();
    let mut is_valid_json = false;
    let mut key_str = String::new();
    let mut keys = Vec::<String>::new();
    let mut is_non_quoted_value = false;
    let mut read_key = false;
    let mut key_val = String::new();

    for val in args.object.chars() {
        if read_key {
            if val == SpecialChars::QUOTE {
                read_key = false;
            } else {
                key_val += val.to_string().as_str();
            }
        }
        if val == SpecialChars::OPEN_BRACE {
            stack_new.push(val);
        } else if val == SpecialChars::QUOTE {
            stack_new.push(val);
            read_key = true;
        }
    }

    for val in args.object.chars() {
        if val == SpecialChars::CLOSE_BRACE && *stack.last().unwrap() == SpecialChars::OPEN_BRACE {
            stack.pop();
            if key_str.len() > 0 {
                keys.push(key_str);
            }
            if stack.len() == 0 {
                is_valid_json = true;
                break;
            } else {
                key_str = String::new();
            }
            continue;
        } else if val == ' ' || val == '\n' || val == '\t' {
            if key_str.len() > 0 {
                key_str += val.to_string().as_str();
            }
            continue;
        } else if val == SpecialChars::QUOTE && *stack.last().unwrap() == SpecialChars::QUOTE {
            if key_str.len() > 0 {
                keys.push(key_str);
                key_str = String::new();
            }
            stack.pop();
            continue;
        } else if val == SpecialChars::QUOTE {
            if *stack.last().unwrap() == SpecialChars::COLON
                || *stack.last().unwrap() == SpecialChars::COMMA
            {
                stack.pop();
            }
            stack.push(val);
            continue;
        } else if !stack.is_empty() && *stack.last().unwrap() == SpecialChars::COLON {
            println!("stack {:?}", stack);
            stack.pop();
            if val == SpecialChars::OPEN_BRACE {
                stack.push(val);
            } else {
                key_str += val.to_string().as_str();
                is_non_quoted_value = true;
            }
            continue;
        } else if !stack.is_empty() && *stack.last().unwrap() == SpecialChars::QUOTE {
            key_str += val.to_string().as_str();
            println!("key_str {:?}", key_str);
            continue;
        } else if val == SpecialChars::COLON {
            if key_str.len() > 0 {
                keys.push(key_str);
                key_str = String::new();
            }
            stack.push(val);
            continue;
        } else if val == SpecialChars::COMMA {
            if key_str.len() > 0 {
                if is_non_quoted_value {
                    if key_str.parse::<i32>().is_ok()
                        || key_str == "true"
                        || key_str == "false"
                        || key_str == "null"
                    {
                        is_non_quoted_value = false;
                        keys.push(key_str);
                        key_str = String::new();
                    } else {
                        is_valid_json = false;
                        break;
                    }
                } else {
                    keys.push(key_str);
                    key_str = String::new();
                }
            }
            stack.push(val);
            continue;
        } else if is_non_quoted_value {
            key_str += val.to_string().as_str();
            continue;
        } else if val.is_alphanumeric() {
            is_valid_json = false;
            break;
        }
        stack.push(val);
    }

    println!("stack: {:?}", stack);
    println!("keys: {:?}", keys);

    println!("Is valid json: {}", is_valid_json);
}

// Tests - Read tests folder iterate and run all Tests
