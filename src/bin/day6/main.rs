mod parser;

use std::{cmp, error::Error};

use bitvec::{bitarr, bits};
use clap::{command, Arg};

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    from: Point,
    to: Point,
}

fn parse_input(input_file: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let input = std::fs::read_to_string(input_file)?;

    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, _>>()?;

    Ok(instructions)
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut grid = bitarr![0; 1000 * 1000];
    let ones = bits![1; 1000];
    let zeros = bits![0; 1000];

    for Instruction { action, from, to } in instructions {
        for y in from.y..=to.y {
            let start = (y * 1000 + from.x).try_into().unwrap();
            let end = (y * 1000 + to.x).try_into().unwrap();

            match action {
                Action::TurnOn => grid[start..=end] |= ones,
                Action::TurnOff => grid[start..=end] &= zeros,
                Action::Toggle => grid[start..=end] ^= ones,
            }
        }
    }

    grid.count_ones()
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut grid = vec![0; 1000 * 1000];

    for Instruction { action, from, to } in instructions {
        for y in from.y..=to.y {
            let start = (y * 1000 + from.x).try_into().unwrap();
            let end = (y * 1000 + to.x).try_into().unwrap();

            match action {
                Action::TurnOn => grid[start..=end].iter_mut().for_each(|x| *x += 1),
                Action::TurnOff => grid[start..=end]
                    .iter_mut()
                    .for_each(|x| *x = cmp::max(0, *x - 1)),
                Action::Toggle => grid[start..=end].iter_mut().for_each(|x| *x += 2),
            }
        }
    }

    grid.iter()
        .map(|x| TryInto::<usize>::try_into(*x).unwrap())
        .sum()
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let instructions = parse_input(input_file).expect("Error parsing input file");

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}
