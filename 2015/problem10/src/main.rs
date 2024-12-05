use std::ops::Add;

const START_NUMBER: &str = "1321131112";
const ITERATIONS: usize = 50;

fn main() {
    // let path = Path::new(FILE_PATH);
    // let display = path.display();

    // let mut file = match File::open(path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // let mut s = String::new();

    // if let Err(why) = file.read_to_string(&mut s) {
    //     panic!("couldn't read {}: {}", display, why)
    // }

    run();
}

fn run() {
    let mut value = String::from(START_NUMBER);
    for i in 0..ITERATIONS {
        value = foo(&value);
    }
    println!("Final output: {}", value.len());
}

fn foo(value: &str) -> String {
    let value = value.to_string();
    let re = fancy_regex::Regex::new(r"(\d)\1*").unwrap();
    let captures = re.captures_iter(&value);
    let mut output = String::from("");
    for capture in captures {
        let capture = capture.unwrap().get(0).unwrap().as_str();
        let count = capture.len();
        let digit = capture.chars().next().unwrap();
        output = output.add(&count.to_string()).add(&digit.to_string());
    }
    output
}
