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
    let mut i = 0;
    while !is_success(i) && i < 100_000_000 {
        i += 1;
    }
    println!("The i with success: {}", i)
}

fn is_success(i: i32) -> bool {
    let hashable = format!("yzbqklnj{}", i);
    let hash = md5::compute(hashable);
    hash[0] == 0 && hash[1] == 0 && hash[2] == 0
}
