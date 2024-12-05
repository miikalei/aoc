use std::collections::HashMap;
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

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }

    run(&s);
}

fn run(input: &str) {
    let mut commands: HashMap<String, Command> = HashMap::new();
    let mut results: HashMap<String, Signal> = HashMap::new();
    for line in input.lines() {
        let command = Command::from(line);
        commands.insert(command.target.to_string(), command);
    }
    println!("Ready!");
    for command in commands.values() {
        println!("{:?}", command)
    }
    let result = commands["a"]
        .command_expression
        .resolve(&commands, &mut results);
    println!("Output signal: {}", result)
}

type Signal = u16;
type Reference = String;

#[derive(Debug)]
struct MyError {}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let (left_side, right_side) = s.split_once("->").unwrap();
        let target: Reference = right_side.trim().to_string();
        let left_side_parts: Vec<&str> = left_side.split_whitespace().collect();

        if left_side_parts.len() == 1 {
            if let Ok(signal) = left_side_parts[0].parse::<u16>() {
                return Command {
                    target,
                    command_expression: CommandExpression::Constant(signal),
                };
            } else {
                return Command {
                    target,
                    command_expression: CommandExpression::ConstantRef(
                        left_side_parts[0].trim().to_string(),
                    ),
                };
            };
        } else if left_side_parts.len() == 2 {
            assert_eq!(left_side_parts[0], "NOT");
            let reference = left_side_parts[1].to_string();
            return Command {
                target,
                command_expression: CommandExpression::Not(reference),
            };
        } else {
            assert_eq!(left_side_parts.len(), 3);
            let ref1 = left_side_parts[0].to_string();
            match left_side_parts[1] {
                "AND" => {
                    let ref2 = left_side_parts[2].to_string();
                    Command {
                        target,
                        command_expression: CommandExpression::And(ref1, ref2),
                    }
                }
                "OR" => {
                    let ref2 = left_side_parts[2].to_string();
                    Command {
                        target,
                        command_expression: CommandExpression::Or(ref1, ref2),
                    }
                }
                "LSHIFT" => {
                    let count = left_side_parts[2].parse().unwrap();
                    Command {
                        target,
                        command_expression: CommandExpression::LShift(ref1, count),
                    }
                }
                "RSHIFT" => {
                    let count = left_side_parts[2].parse().unwrap();
                    Command {
                        target,
                        command_expression: CommandExpression::Rshift(ref1, count),
                    }
                }
                other => panic!("Unknown binary operator {other}."),
            }
        }
    }
}

#[derive(Debug)]
struct Command {
    target: Reference,
    command_expression: CommandExpression,
}

#[derive(Debug)]
enum CommandExpression {
    Constant(Signal),
    ConstantRef(Reference),
    And(Reference, Reference),
    Or(Reference, Reference),
    Rshift(Reference, i32),
    LShift(Reference, i32),
    Not(Reference),
}

impl CommandExpression {
    fn resolve(
        &self,
        commands: &HashMap<String, Command>,
        results: &mut HashMap<String, Signal>,
    ) -> Signal {
        println!("Total amount of computations, {}", results.values().count());
        match self {
            CommandExpression::Constant(signal) => *signal,
            CommandExpression::ConstantRef(reference) => {
                resolve_reference(reference, commands, results)
            }
            CommandExpression::Not(reference) => {
                let pre_signal = resolve_reference(reference, commands, results);
                !pre_signal
            }
            CommandExpression::And(ref1, ref2) => {
                let signal1 = resolve_reference(ref1, commands, results);
                let signal2 = resolve_reference(ref2, commands, results);
                signal1 & signal2
            }
            CommandExpression::Or(ref1, ref2) => {
                let signal1 = resolve_reference(ref1, commands, results);
                let signal2 = resolve_reference(ref2, commands, results);
                signal1 | signal2
            }
            CommandExpression::LShift(ref1, amount) => {
                let signal1 = resolve_reference(ref1, commands, results);
                signal1 << amount
            }
            CommandExpression::Rshift(ref1, amount) => {
                let signal1 = resolve_reference(ref1, commands, results);
                signal1 >> amount
            }
        }
    }
}

fn resolve_reference(
    reference: &str,
    commands: &HashMap<String, Command>,
    results: &mut HashMap<String, Signal>,
) -> Signal {
    if let Some(signal) = results.get(reference) {
        return *signal;
    }
    if let Some(command) = commands.get(reference) {
        let signal = command.command_expression.resolve(commands, results);
        results.insert(reference.to_string(), signal);
        return signal;
    }
    reference.parse().unwrap()
}
