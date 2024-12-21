use std::{collections::HashSet, fs::File, io::Read, path::Path, str::FromStr};

fn main() {
    let rules_s = read_input("rules.txt");
    let rules: Vec<ReplacementRule> = rules_s.lines().map(|x| x.into()).collect();

    let molecule_s = read_input("molecule.txt");
    let start_molecule = Molecule::new(molecule_s);

    let target_molecule = Molecule {
        string: "e".to_string(),
    };

    let mut steps = 0;
    let mut molecules: HashSet<Molecule> = HashSet::new();
    molecules.insert(start_molecule);
    while !molecules.contains(&target_molecule) {
        println!("Step {steps}, there are {} options", molecules.len());
        steps += 1;
        let mut new_molecule_set = HashSet::new();
        for m in molecules.iter() {
            m.apply_replacements(&rules).into_iter().for_each(|m| {
                new_molecule_set.insert(Molecule { string: m });
            });
        }
        let mut molecule_set_vec: Vec<Molecule> = new_molecule_set.into_iter().collect();

        // Only keep 100 shortest around, greedy try to get around the exponential search space
        molecule_set_vec.sort_by(|a, b| a.string.len().cmp(&b.string.len()));
        let shortest_molecules: Vec<Molecule> = molecule_set_vec.into_iter().take(100).collect();
        let mut new_molecule_set: HashSet<Molecule> = HashSet::new();
        shortest_molecules.iter().for_each(|m| {
            new_molecule_set.insert(m.clone());
        });
        molecules = new_molecule_set;

        molecules.iter().for_each(|m| println!("{:?}", m));
    }

    println!("Found the target molecule after {} steps", steps)
}

#[derive(Debug)]
struct ReplacementRule {
    source: String,
    target: String,
}

impl FromStr for ReplacementRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (source, target) = s.split_once("=>").unwrap();
        Ok(Self {
            // Note: inverted for part 2
            target: source.trim().to_string(),
            source: target.trim().to_string(),
        })
    }
}
impl From<&str> for ReplacementRule {
    fn from(value: &str) -> Self {
        value.parse().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Molecule {
    string: String,
}

impl Molecule {
    pub fn new(string: String) -> Self {
        Self { string }
    }

    fn apply_replacement(&self, replacement: &ReplacementRule) -> HashSet<String> {
        self.string
            .match_indices(&replacement.source)
            .map(|(match_index, _)| {
                let mut orig = self.string.clone();
                orig.replace_range(
                    match_index..match_index + replacement.source.len(),
                    &replacement.target,
                );
                orig
            })
            .collect()
    }

    fn apply_replacements(&self, replacements: &Vec<ReplacementRule>) -> HashSet<String> {
        replacements
            .iter()
            .flat_map(|r| self.apply_replacement(r))
            .collect()
    }
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
