use clap::{command, Arg};
use itertools::Itertools;

fn solve(items: Vec<u64>, num_groups: usize) -> u64 {
    let target = items.iter().sum::<u64>() / num_groups as u64;

    for group_size in 1..items.len() {
        let min_entanglement = items
            .iter()
            .combinations(group_size)
            .filter(|group| group.iter().copied().sum::<u64>() == target)
            .map(|group| group.iter().copied().product())
            .min();

        if let Some(entanglement) = min_entanglement {
            return entanglement;
        }
    }

    panic!("No solution found");
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let numbers = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .lines()
        .map(|line| line.parse::<u64>().expect("Error parsing number"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", solve(numbers.clone(), 3));
    println!("Part 2: {}", solve(numbers, 4));
}
