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
    pub const OPEN_SQUARE_BRACE: char = '[';
    pub const CLOSE_BRACE: char = '}';
    pub const CLOSE_SQUARE_BRACE: char = ']';
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

fn is_valid_json<I>(json_iter: &mut I, val_type_input: JsonValueType) -> bool
where
    I: Iterator<Item = char>,
{
    let mut stack = Vec::<char>::new();
    let mut stack_new = Vec::<char>::new();
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
    let mut last_val_comma = false;

    if val_type_input == JsonValueType::ARRAY {
        stack_new.push(SpecialChars::OPEN_SQUARE_BRACE);
        read_val = true;
    }
    // let mut iter = json_iter;
    loop {
        let val;
        if let Some(v) = json_iter.next() {
            val = v;
            if val_type_input == JsonValueType::ARRAY
                && val == SpecialChars::CLOSE_SQUARE_BRACE
                && stack_new.len() == 1
            {
                break;
            }
            println!("val_start: {:?}", val);
        } else {
            break;
        }
        if read_key {
            last_val_comma = false;
            println!("val_ready_key: {:?}", val);
            if val == SpecialChars::QUOTE {
                println!("key_val_top: {:?}", key_val);
                read_key = false;
                key_terminated = true;
                stack_new.pop();
            } else {
                key_val += val.to_string().as_str();
            }
            continue;
        }
        if key_terminated {
            println!("key_val: {:?}", key_val);
            if val == SpecialChars::COLON {
                read_val = true;
                key_terminated = false;
            } else if val == ' ' || val == '\n' || val == '\t' {
                continue;
            } else {
                invalid_json = true;
                break;
            }
            continue;
        }
        if read_val {
            if val == SpecialChars::QUOTE {
                stack_new.push(val);
            } else if val.is_numeric() {
                val_type = JsonValueType::NUMBER;
                val_val += val.to_string().as_str();
            } else if val == SpecialChars::OPEN_BRACE {
                val_type = JsonValueType::OBJECT;
                let ans = is_valid_json(json_iter.by_ref(), val_type);
                if ans {
                    key_val = String::new();
                    val_val = String::new();
                    val_type = JsonValueType::STRING;
                    read_val = false;
                    continue;
                    // index to be updated - get back same iter?
                } else {
                    invalid_json = true;
                    break;
                }
                // send to this function recursively from this index
            } else if val == SpecialChars::OPEN_SQUARE_BRACE {
                val_type = JsonValueType::ARRAY;
                let ans = is_valid_json(json_iter.by_ref(), val_type);
                println!("ans: {:?}", ans);
                if ans {
                    key_val = String::new();
                    val_val = String::new();
                    val_type = JsonValueType::STRING;
                    read_val = false;
                    continue;
                    // index to be updated - get back same iter?
                } else {
                    invalid_json = true;
                    break;
                }
                /*if is_valid_json(&json[ind..]) {
                    // index to be updated - get back same iter?
                } else {
                    invalid_json = true;
                    break;
                }*/
            } else if val == 't' || val == 'f' {
                val_type = JsonValueType::BOOLEAN;
                val_val += val.to_string().as_str();
            } else if val == 'n' {
                val_type = JsonValueType::NULL;
                val_val += val.to_string().as_str();
            } else if val == ' ' || val == '\n' || val == '\t' {
                val_val += val.to_string().as_str();
                continue;
            } else {
                invalid_json = true;
                break;
            }
            val_parse = true;
            read_val = false;
            continue;
        }
        if val_parse {
            if val == SpecialChars::COMMA
                || val == SpecialChars::CLOSE_BRACE
                || val == SpecialChars::CLOSE_SQUARE_BRACE
            {
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
                if val_type_input == JsonValueType::ARRAY {
                    read_val = true;
                }
                val_type = JsonValueType::STRING;
                val_val = String::new();
                key_val = String::new();
                if val == SpecialChars::COMMA {
                    last_val_comma = true;
                }
                if val == SpecialChars::CLOSE_BRACE || val == SpecialChars::CLOSE_SQUARE_BRACE {
                    break;
                }
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
        if val == SpecialChars::OPEN_BRACE || val == SpecialChars::OPEN_SQUARE_BRACE {
            stack_new.push(val);
        } else if val == SpecialChars::QUOTE && val_type_input != JsonValueType::ARRAY {
            stack_new.push(val);
            println!("stack_new_last: {:?}", stack_new);
            println!("key_val_end: {:?}", key_val);
            read_key = true;
        } else if val == SpecialChars::CLOSE_BRACE {
            if last_val_comma {
                println!("Hello");
                invalid_json = true;
                break;
            }
        }
    }

    println!("stack: {:?}", stack);
    println!("keys: {:?}", keys);

    println!("Is invalid json: {}", invalid_json);
    println!("map_all: {:?}", map_all);
    println!("stack_new: {:?}", stack_new);
    return !invalid_json;
}

// Tests - Read tests folder iterate and run all Tests

fn main() {
    let args = Args::parse();
    let mut json = args.object.chars();
    let mut ans = is_valid_json(json.by_ref(), JsonValueType::OBJECT);
    if args.object == "" {
        ans = false;
    }
    println!("Is valid json: {}", ans);
}
