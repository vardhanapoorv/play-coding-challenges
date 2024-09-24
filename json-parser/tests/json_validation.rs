use std::fs::{self, read_to_string};
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_all_steps() {
    let steps = vec!["step1"];
    for step in steps {
        let json_files = get_json_files_in_dir(step);
        println!("json_files: {:?}", json_files);
        for file in json_files {
            let json_content = read_json_file(&file);
            let output = run_json_validator(&json_content);

            if file.file_name().unwrap() == "valid.json" {
                assert!(
                    output.contains("Is valid json: true"),
                    "Expected valid JSON to pass."
                );
            } else if file.file_name().unwrap() == "invalid.json" {
                assert!(
                    output.contains("Is valid json: false"),
                    "Expected invalid JSON to fail."
                );
            }
        }
    }
}

fn get_json_files_in_dir(step: &str) -> Vec<PathBuf> {
    let mut path = PathBuf::from(format!("tests/{}", step));
    fs::read_dir(path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to get directory entry");
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect()
}

fn read_json_file(filepath: &PathBuf) -> String {
    read_to_string(filepath).expect("Failed to read JSON file")
}

fn run_json_validator(json: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_json-parser"))
        .arg("--object")
        .arg(json)
        .output()
        .expect("Failed to execute process");

    String::from_utf8_lossy(&output.stdout).to_string()
}
