use std::{fs::File, io::Read, path::Path};

const TARGET: u32 = 150;

fn main() {
    let s = read_input();
    let mut container_sizes: Vec<u32> = s.lines().map(|x| x.parse().unwrap()).collect();
    container_sizes.sort_unstable();
    container_sizes.reverse();

    // Part 1
    let mut matches = 0;
    for i in 0..=2_u32.pow(container_sizes.len().try_into().unwrap()) {
        let sum = calculate_sum(i, &container_sizes);
        if sum == TARGET {
            matches += 1;
        }
    }
    println!("Number of matches: {}", matches);

    // Part 2
    let mut min_container_count = u32::MAX;
    for i in 0..=2_u32.pow(container_sizes.len().try_into().unwrap()) {
        let sum = calculate_sum(i, &container_sizes);
        if sum == TARGET {
            let container_count = i.count_ones();
            if container_count < min_container_count {
                min_container_count = container_count
            }
        }
    }
    println!("min_container_count: {}", min_container_count);

    let mut matches = 0;
    for i in 0..=2_u32.pow(container_sizes.len().try_into().unwrap()) {
        let sum = calculate_sum(i, &container_sizes);
        if sum == TARGET && i.count_ones() == min_container_count {
            matches += 1
        }
    }
    println!("Number of minimum container count matches: {}", matches)
}

fn calculate_sum(weights: u32, container_sizes: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 0..32 {
        let b = weights >> i & 1;
        let c = container_sizes.get(i).unwrap_or(&0);
        sum += b * c;
    }
    sum
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
