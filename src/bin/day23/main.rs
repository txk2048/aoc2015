use clap::{command, Arg};

mod parser;

enum TargetRegister {
    A,
    B,
}

enum Instruction {
    Half(TargetRegister),
    Triple(TargetRegister),
    Increment(TargetRegister),
    Jump(i32),
    JumpIfEven(TargetRegister, i32),
    JumpIfOne(TargetRegister, i32),
}

fn run(a_value: u32, instructions: &Vec<Instruction>) -> u32 {
    // a and b registers
    let mut a = a_value;
    let mut b = 0;

    // instruction pointer
    let mut ip = 0;

    while ip < instructions.len() {
        match instructions[ip] {
            Instruction::Half(TargetRegister::A) => a /= 2,
            Instruction::Half(TargetRegister::B) => b /= 2,
            Instruction::Triple(TargetRegister::A) => a *= 3,
            Instruction::Triple(TargetRegister::B) => b *= 3,
            Instruction::Increment(TargetRegister::A) => a += 1,
            Instruction::Increment(TargetRegister::B) => b += 1,
            Instruction::Jump(offset) => {
                ip = (ip as i32 + offset) as usize;
                continue;
            }
            Instruction::JumpIfEven(TargetRegister::A, offset) => {
                if a % 2 == 0 {
                    ip = (ip as i32 + offset) as usize;
                    continue;
                }
            }
            Instruction::JumpIfEven(TargetRegister::B, offset) => {
                if b % 2 == 0 {
                    ip = (ip as i32 + offset) as usize;
                    continue;
                }
            }
            Instruction::JumpIfOne(TargetRegister::A, offset) => {
                if a == 1 {
                    ip = (ip as i32 + offset) as usize;
                    continue;
                }
            }
            Instruction::JumpIfOne(TargetRegister::B, offset) => {
                if b == 1 {
                    ip = (ip as i32 + offset) as usize;
                    continue;
                }
            }
        }

        ip += 1;
    }

    b
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let instructions = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .lines()
        .map(|line| {
            line.trim()
                .parse::<Instruction>()
                .expect("Error parsing instruction")
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", run(0, &instructions));
    println!("Part 2: {}", run(1, &instructions));
}
