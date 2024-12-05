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
    let ret = run(&s);
    println!("Total paper: {}", ret);
}

fn run(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let split_line: Vec<&str> = line.split('x').collect();
        let x: i32 = split_line[0].parse().unwrap();
        let y: i32 = split_line[1].parse().unwrap();
        let z: i32 = split_line[2].parse().unwrap();
        let paper = present_paper(x, y, z);
        let ribbon = present_ribbon(x, y, z);
        total += ribbon;
    }
    total
}

fn present_paper(x: i32, y: i32, z: i32) -> i32 {
    let hull = 2 * x * y + 2 * y * z + 2 * z * x;
    let parts = vec![x * y, y * z, z * x];
    let slack = parts.iter().min().unwrap();
    hull + slack
}

fn present_ribbon(x: i32, y: i32, z: i32) -> i32 {
    let mut sides = [x, y, z];
    sides.sort();
    let min1 = sides[0];
    let min2 = sides[1];
    let perimeter = min1 + min1 + min2 + min2;
    let volume = x * y * z;
    volume + perimeter
}
