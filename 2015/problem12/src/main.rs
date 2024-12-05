use std::{fs::File, io::Read, path::Path};

use serde_json::Value;

const START_INPUT: &str = "vzbxxzaa";
const FILE_PATH: &str = "input.txt";

fn main() {
    let path = Path::new(FILE_PATH);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }

    run(s);
}

fn run(s: String) {
    let v: Value = serde_json::from_str(&s).unwrap();
    let total = calculate_total(&v);
    println!("Total: {}", total);
}

fn calculate_total(v: &Value) -> i32 {
    match v {
        Value::Null => 0,
        Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap() as i32,
        Value::Array(v) => v.iter().map(calculate_total).sum(),
        Value::Bool(_) => 0,
        Value::Object(map) => {
            if map.values().any(|v| v == "red") {
                0
            } else {
                map.values().map(calculate_total).sum()
            }
        }
    }
}
