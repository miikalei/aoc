use std::{fs::File, io::Read, path::Path};

fn main() {
    let s = read_input();
    let mut a = [false; 100 * 100];
    for (i, c) in s.chars().filter(|&c| c == '.' || c == '#').enumerate() {
        match c {
            '.' => a[i] = false,
            '#' => a[i] = true,
            _ => (),
        }
    }

    let mut lf = LightField::from(a);
    for i in 0..100 {
        lf = lf.next();
    }
    println!("Light count after 100 steps: {}", lf.count())
}

#[derive(Debug)]
struct LightField {
    lights: [bool; 100 * 100],
}

impl LightField {
    pub fn new() -> Self {
        Self {
            lights: [false; 100 * 100],
        }
    }

    pub fn from(mut lights: [bool; 100 * 100]) -> Self {
        /* Part 2 modification */
        lights[LightField::coord_index(0, 0)] = true;
        lights[LightField::coord_index(0, 99)] = true;
        lights[LightField::coord_index(99, 0)] = true;
        lights[LightField::coord_index(99, 99)] = true;
        /* Part 2 modification ends */

        Self { lights }
    }

    pub fn next(&self) -> Self {
        let mut next_lights = [false; 100 * 100];
        for x in 0..100 {
            for y in 0..100 {
                let next_is_on = self.get_next(x, y);
                next_lights[LightField::coord_index(x, y)] = next_is_on;
            }
        }

        /* Part 2 modification */
        next_lights[LightField::coord_index(0, 0)] = true;
        next_lights[LightField::coord_index(0, 99)] = true;
        next_lights[LightField::coord_index(99, 0)] = true;
        next_lights[LightField::coord_index(99, 99)] = true;
        /* Part 2 modification ends */
        Self {
            lights: next_lights,
        }
    }

    pub fn count(&self) -> usize {
        self.lights.iter().filter(|b| **b).count()
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        if !LightField::is_valid_coordinate(x, y) {
            return false;
        }
        self.lights[LightField::coord_index(x, y)]
    }

    fn get_next(&self, x: i32, y: i32) -> bool {
        let current_value = self.get(x, y);
        let neighbor_count = self.count_neighbours(x, y);
        if current_value {
            return neighbor_count == 2 || neighbor_count == 3;
        } else {
            return neighbor_count == 3;
        }
    }

    fn coord_index(x: i32, y: i32) -> usize {
        (100 * y + x).try_into().unwrap()
    }

    fn count_neighbours(&self, x: i32, y: i32) -> u32 {
        let neighbours = LightField::get_neighbours(x, y);
        let mut count = 0;
        for neighbor in neighbours {
            let is_lit = self.get(neighbor.0, neighbor.1);
            if is_lit {
                count += 1;
            }
        }
        count
    }

    fn is_valid_coordinate(x: i32, y: i32) -> bool {
        (0..100).contains(&x) && (0..100).contains(&y)
    }

    fn get_neighbours(x: i32, y: i32) -> Vec<(i32, i32)> {
        let neighbours = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        neighbours
            .into_iter()
            .filter(|coord| LightField::is_valid_coordinate(coord.0, coord.1))
            .collect()
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
