const START_INPUT: &str = "vzbxxzaa";

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
    let mut password = String::from(START_INPUT);
    while !is_good(&password) {
        let codes = string_to_codes(&password);
        let codes_inc = increment(&codes);
        password = codes_to_string(&codes_inc);
    }
    println!("Passed password: {}", &password);
}

fn is_good(value: &str) -> bool {
    contains_streak(value) && contains_no_forbidden(value) && contains_two_pairs(value)
}

fn contains_streak(value: &str) -> bool {
    value.contains("abc")
        || value.contains("bcd")
        || value.contains("cde")
        || value.contains("def")
        || value.contains("efg")
        || value.contains("fgh")
        || value.contains("ghi")
        || value.contains("hij")
        || value.contains("ijk")
        || value.contains("jkl")
        || value.contains("klm")
        || value.contains("lmn")
        || value.contains("mno")
        || value.contains("nop")
        || value.contains("opq")
        || value.contains("pqr")
        || value.contains("qrs")
        || value.contains("rst")
        || value.contains("stu")
        || value.contains("tuv")
        || value.contains("uvw")
        || value.contains("vwx")
        || value.contains("wxy")
        || value.contains("xyz")
}
fn contains_no_forbidden(value: &str) -> bool {
    !value.contains('i') && !value.contains('o') && !value.contains('l')
}
fn contains_two_pairs(value: &str) -> bool {
    let mut pair_count = 0;
    for i in 0..26 {
        let c = codes_to_string(&vec![i as u8; 1]).chars().next().unwrap();
        let s = format!("{}{}", c, c);
        if value.contains(&s) {
            pair_count += 1;
        }
    }
    pair_count >= 2
}

fn string_to_codes(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|char| ((char as u8) & 0b0001_1111) - 1)
        .collect()
}

fn increment(input: &Vec<u8>) -> Vec<u8> {
    let mut carry = true;
    let mut output: Vec<u8> = Vec::new();
    for char_code in input.iter().rev() {
        if !carry {
            output.insert(0, *char_code)
        } else if char_code + 1 < 26 {
            carry = false;
            output.insert(0, *char_code + 1)
        } else {
            output.insert(0, 0)
        }
    }
    assert_eq!(carry, false);
    output
}

fn codes_to_string(input: &Vec<u8>) -> String {
    input
        .iter()
        .map(|char_code| ((char_code + 1) | 0b0110_0000) as char)
        .collect()
}
