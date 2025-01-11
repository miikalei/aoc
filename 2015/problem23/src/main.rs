use std::{fs::File, io::Read, path::Path, str::FromStr};

fn main() {
    let input = read_input("input.txt");
    let commands: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut computer = Computer::new();
    computer.run_commands(&commands);
}

#[derive(Debug)]
struct Computer {
    reg_a: u32,
    reg_b: u32,

    step_counter: usize,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            reg_a: 0,
            reg_b: 0,
            step_counter: 0,
        }
    }

    pub fn run_commands(&mut self, instructions: &[Instruction]) {
        loop {
            if let Some(next_instruction) = instructions.get(self.step_counter) {
                println!("Running instruction at {}", self.step_counter);
                self.run_instruction(next_instruction);
            } else {
                break;
            }
        }
        println!("{:?}", self);
    }

    fn get_register(&mut self, register: &Register) -> &mut u32 {
        match register {
            Register::A => &mut self.reg_a,
            Register::B => &mut self.reg_b,
        }
    }

    fn step(&mut self) {
        self.step_counter += 1
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Hlf(r) => {
                let r = self.get_register(r);
                *r /= 2;
                self.step();
            }
            Instruction::Tpl(r) => {
                let r = self.get_register(r);
                *r *= 3;
                self.step();
            }
            Instruction::Inc(r) => {
                let r = self.get_register(r);
                *r += 1;
                self.step();
            }
            Instruction::Jmp(offset) => {
                self.step_counter = self.step_counter.saturating_add_signed(*offset)
            }
            Instruction::Jie(r, offset) => {
                let r = self.get_register(r);
                if *r % 2 == 0 {
                    self.step_counter = self.step_counter.saturating_add_signed(*offset)
                } else {
                    self.step();
                }
            }
            Instruction::Jio(r, offset) => {
                let r = self.get_register(r);
                if *r == 1 {
                    self.step_counter = self.step_counter.saturating_add_signed(*offset)
                } else {
                    self.step();
                }
            }
        }
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, rest) = s.split_once(" ").unwrap();
        match cmd.trim() {
            "hlf" => {
                let r: Register = rest.parse().unwrap();
                Ok(Instruction::Hlf(r))
            }
            "tpl" => {
                let r: Register = rest.parse().unwrap();
                Ok(Instruction::Tpl(r))
            }
            "inc" => {
                let r: Register = rest.parse().unwrap();
                Ok(Instruction::Inc(r))
            }
            "jmp" => {
                let offset: isize = rest.trim().parse().unwrap();
                Ok(Instruction::Jmp(offset))
            }
            "jie" => {
                let (r, offset) = rest.split_once(',').unwrap();
                let r: Register = r.parse().unwrap();
                let offset: isize = offset.trim().parse().unwrap();
                Ok(Instruction::Jie(r, offset))
            }
            "jio" => {
                let (r, offset) = rest.split_once(',').unwrap();
                let r: Register = r.parse().unwrap();
                let offset: isize = offset.trim().parse().unwrap();
                Ok(Instruction::Jio(r, offset))
            }
            _ => panic!(),
        }
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
