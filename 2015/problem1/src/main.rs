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
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }
    run(&s);
}

fn run(input: &str) {
    let mut total = 0;
    for (index, char) in input.chars().enumerate() {
        match char {
            '(' => total += 1,
            ')' => total -= 1,
            other => panic!("Unknown character: {}", other),
        }
        if total < 0 {
            return println!("Basement visited on charater in index {}", index + 1);
        }
    }
    println!("No basement found.")
}
