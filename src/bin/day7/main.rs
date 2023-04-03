mod parser;

use std::{collections::HashMap, error::Error};

use clap::{command, Arg};

enum Operation {
    Assign,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

enum Operand {
    Value(u16),
    Wire(String),
}

struct Instruction {
    op: Operation,
    arg1: Operand,
    arg2: Option<Operand>,
    output: String,
}

fn parse_input(input_file: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let input = std::fs::read_to_string(input_file)?;

    let instructions = input
        .trim()
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok(instructions)
}

fn eval(wire: &str, instructions: &[Instruction], wires: &mut HashMap<String, u16>) -> Option<u16> {
    // wire is in cache
    if let Some(value) = wires.get(wire) {
        return Some(*value);
    }

    // find corresponding instruction
    let instruction = instructions.iter().find(|i| i.output == wire)?;

    // evaluate arguments
    let left = match &instruction.arg1 {
        Operand::Value(v) => *v,
        Operand::Wire(w) => eval(w, instructions, wires)?,
    };

    let right = match &instruction.arg2 {
        Some(Operand::Value(v)) => *v,
        Some(Operand::Wire(w)) => eval(w, instructions, wires)?,
        None => 0,
    };

    // evaluate instruction
    let value = match instruction.op {
        Operation::Assign => left,
        Operation::And => left & right,
        Operation::Or => left | right,
        Operation::LShift => left << right,
        Operation::RShift => left >> right,
        Operation::Not => !left,
    };

    wires.insert(wire.to_string(), value);

    Some(value)
}

fn part1(instructions: &[Instruction]) -> Option<u16> {
    eval("a", instructions, &mut HashMap::new())
}

fn part2(instructions: &[Instruction]) -> Option<u16> {
    let a = part1(instructions)?;

    let mut wires: HashMap<String, u16> = HashMap::new();
    wires.insert("b".to_string(), a);

    eval("a", instructions, &mut wires)
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let instructions = parse_input(&input_file).expect("Error parsing input file");

    match part1(&instructions) {
        Some(value) => println!("Part 1: {}", value),
        None => println!("Part 1: no solution"),
    }

    match part2(&instructions) {
        Some(value) => println!("Part 2: {}", value),
        None => println!("Part 2: no solution"),
    }
}
