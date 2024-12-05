use std::fs::File;
use std::io::Read;
use std::path::Path;

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

    run(&s);
}

fn run(input: &str) {
    let mut code_chars = 0;
    let mut chars = 0;
    for line in input.lines() {
        code_chars += line.len();
        println!("Before: {} ({})", line, line.len());
        let temp = line.replace(r"\", r"\\").replace("\"", "\\\"");
        let result = format!("\"{}\"", temp);
        println!("After: {} ({})", result, result.len());
        chars += result.len();
    }
    println!("Difference in lengts: {}", chars - code_chars)
}
