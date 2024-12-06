use std::{collections::HashMap, fs::File, io::Read, path::Path};

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

    for i in partition(5, 3) {
        println!("{:?}", i)
    }
}

fn run(s: &str) {
    let mut ingredients = Vec::new();

    for line in s.lines() {
        let (name, rest) = line.split_once(':').unwrap();
        let ingredient = Ingredient::new(name, rest);

        ingredients.push(ingredient);
    }

    for i in ingredients {
        println!("{:?}", i);
    }
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    properties: HashMap<String, i32>,
}

impl Ingredient {
    pub fn new(name: &str, ingredient_string: &str) -> Self {
        let properties_array = ingredient_string.split(',');
        let mut properties = HashMap::new();
        for prop in properties_array {
            let (prop_name, prop_number) = prop.trim().split_once(' ').unwrap();
            properties.insert(prop_name.to_string(), prop_number.parse().unwrap());
        }
        Ingredient {
            name: name.to_string(),
            properties,
        }
    }
}

fn partition(total: u32, n: u32) -> Vec<Vec<u32>> {
    let mut stack: Vec<(u32, u32, Vec<u32>)> = vec![(total, n, Vec::new())];
    let mut results = Vec::new();

    while let Some((remaining, parts, mut current)) = stack.pop() {
        if parts == 1 {
            current.push(remaining);
            results.push(current);
            continue;
        }

        for i in 0..=remaining {
            let mut new_current = current.clone();
            new_current.push(i);
            stack.push((remaining - i, parts - 1, new_current));
        }
    }
    results
}
