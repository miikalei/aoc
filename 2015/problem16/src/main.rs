use std::{collections::HashMap, fs::File, io::Read, path::Path};

fn main() {
    let s = read_input();
    let mut sues = Vec::new();

    for line in s.lines() {
        let (prefix, suffix) = line.split_once(':').unwrap();
        let number: u32 = prefix.strip_prefix("Sue ").unwrap().parse().unwrap();
        sues.push(Sue::new(number, suffix));
    }

    let descriptor: HashMap<&str, u32> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let m = sues.iter().find(|&sue| {
        sue.descriptors.iter().all(|(prop_name, count)| {
            let sue_count = descriptor.get(&prop_name.as_str()).unwrap();
            match prop_name.as_str() {
                "cats" | "trees" => count > sue_count,
                "pomeranians" | "goldfish" => count < sue_count,
                _ => sue_count == count,
            }
        })
    });

    println!("{:?}", m)
}

#[derive(Debug)]
struct Sue {
    number: u32,
    descriptors: HashMap<String, u32>,
}

impl Sue {
    pub fn new(number: u32, descriptor: &str) -> Self {
        let mut descriptors = HashMap::new();
        for fact in descriptor.split(',') {
            let (name, count) = fact.trim().split_once(':').unwrap();
            descriptors.insert(name.to_string(), count.trim().parse().unwrap());
        }

        Sue {
            number,
            descriptors,
        }
    }
}

const FILE_PATH: &str = "input.txt";
fn read_input() -> String {
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
    s
}
