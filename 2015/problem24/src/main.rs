use itertools::{self, Itertools};
use std::{fs::File, io::Read, path::Path};

fn main() {
    let input = read_input("input.txt");
    let packages: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let group_sum_target: i64 = packages.iter().sum::<i64>() / 4; // 3; // Part 2 modification

    let mut record_container_count = usize::MAX;
    let mut record_qe = i64::MAX;

    for a_packages in partition_iter(&packages, group_sum_target) {
        if a_packages.len() <= record_container_count {
            record_container_count = a_packages.len();
            let qe = qe(&a_packages.iter().copied().copied().collect::<Vec<_>>());
            if qe < record_qe {
                record_qe = qe
            }
        }
    }
    println!("Iteration done, record qe was {}", record_qe);
}

fn qe(input: &[i64]) -> i64 {
    input.iter().product()
}

fn partition_iter(input: &[i64], sum_target: i64) -> impl Iterator<Item = std::vec::Vec<&'_ i64>> {
    input
        .iter()
        .powerset()
        .filter(move |subset| subset.iter().copied().sum::<i64>() == sum_target)
}

fn read_input(file_path: &str) -> String {
    let path = Path::new(file_path);
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
