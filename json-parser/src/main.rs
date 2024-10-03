// cli using clap
use clap::Parser;
use std::collections::HashMap;

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

#[derive(PartialEq, Debug)]
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
    let mut invalid_json = false;
    let mut key_str = String::new();
    let mut keys = Vec::<String>::new();
    let mut is_non_quoted_value = false;
    let mut read_key = false;
    let mut read_val = false;
    let mut key_val = String::new();
    let mut val_val = String::new();
    let mut val_type = JsonValueType::STRING;
    let mut val_parse = false;
    let mut key_terminated = false;
    let mut val_next = false;
    let mut map_all = HashMap::new();

    for val in args.object.chars() {
        if read_key {
            if val == SpecialChars::QUOTE {
                println!("key_val_top: {:?}", key_val);
                read_key = false;
                key_terminated = true;
                stack_new.pop();
            } else {
                key_val += val.to_string().as_str();
            }
            continue;
            println!("should not reach here");
        }
        if key_terminated {
            println!("key_val: {:?}", key_val);
            if val == SpecialChars::COLON {
                val_next = true;
                key_terminated = false;
            } else if val == ' ' || val == '\n' || val == '\t' {
                continue;
            } else {
                invalid_json = true;
                break;
            }
            continue;
        }
        if val_next {
            if val == ' ' || val == '\n' || val == '\t' {
                continue;
            } else {
                val_next = false;
                read_val = true;
            }
        }
        if read_val {
            if val == SpecialChars::QUOTE {
                stack_new.push(val);
            } else if val.is_numeric() {
                val_type = JsonValueType::NUMBER;
                val_val += val.to_string().as_str();
            } else if val == SpecialChars::OPEN_BRACE {
                val_type = JsonValueType::OBJECT;
                // send to this function recursively from this index
            } else if val == SpecialChars::OPEN_BRACE {
                val_type = JsonValueType::ARRAY;
            } else if val == 't' || val == 'f' {
                val_type = JsonValueType::BOOLEAN;
                val_val += val.to_string().as_str();
            } else if val == 'n' {
                val_type = JsonValueType::NULL;
                val_val += val.to_string().as_str();
            } else {
                invalid_json = true;
                break;
            }
            val_parse = true;
            read_val = false;
            continue;
        }
        if val_parse {
            if val == SpecialChars::COMMA || val == SpecialChars::CLOSE_BRACE {
                val_val = val_val.trim().to_string();
                println!("val_val: {:?}", val_val);
                if val_type == JsonValueType::NUMBER {
                    if val_val.parse::<i32>().is_err() {
                        invalid_json = true;
                        break;
                    }
                } else if val_type == JsonValueType::BOOLEAN {
                    if val_val != "true" && val_val != "false" {
                        invalid_json = true;
                        break;
                    }
                } else if val_type == JsonValueType::NULL {
                    if val_val != "null" {
                        invalid_json = true;
                        break;
                    }
                }
                map_all.insert(key_val.clone(), val_val.clone());
                val_parse = false;
                val_type = JsonValueType::STRING;
                val_val = String::new();
                key_val = String::new();
                continue;
            } else {
                if val_type == JsonValueType::STRING {
                    if val == SpecialChars::QUOTE {
                        stack_new.pop();
                    } else {
                        println!("valwithin: {:?}", val);
                        val_val += val.to_string().as_str();
                    }
                } else if val_type == JsonValueType::NUMBER {
                    if val.is_numeric() {
                        val_val += val.to_string().as_str();
                    }
                } else if val_type == JsonValueType::OBJECT {
                    // send to this function recursively from this index
                } else if val_type == JsonValueType::ARRAY {
                    // send to this function recursively from this index
                } else if val_type == JsonValueType::BOOLEAN {
                    val_val += val.to_string().as_str();
                } else if val_type == JsonValueType::NULL {
                    val_val += val.to_string().as_str();
                }
                continue;
            }
        }
        if val == SpecialChars::OPEN_BRACE {
            stack_new.push(val);
        } else if val == SpecialChars::QUOTE {
            stack_new.push(val);
            println!("stack_new_last: {:?}", stack_new);
            println!("key_val_end: {:?}", key_val);
            read_key = true;
        }
    }

    /*
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
    }*/

    println!("stack: {:?}", stack);
    println!("keys: {:?}", keys);

    println!("Is valid json: {}", is_valid_json);
    println!("Is invalid json: {}", invalid_json);
    println!("map_all: {:?}", map_all);
    println!("stack_new: {:?}", stack_new);
}

// Tests - Read tests folder iterate and run all Tests
