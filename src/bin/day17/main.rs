use clap::{command, Arg};
use itertools::Itertools;

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let containers = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .trim()
        .lines()
        .map(|line| line.parse::<u32>().expect("Error parsing line"))
        .collect::<Vec<u32>>();

    let sum = |combination: &Vec<&u32>| combination.iter().map(|x| *x).sum::<u32>();

    let combination_lengths: Vec<usize> = (1..=containers.len())
        .into_iter()
        .flat_map(|n| {
            containers.iter().combinations(n).filter_map(|combination| {
                if sum(&combination) == 150 {
                    Some(combination.len())
                } else {
                    None
                }
            })
        })
        .collect();

    println!("Part 1: {}", combination_lengths.len());

    let min_length = combination_lengths.iter().min().unwrap();
    let result2 = combination_lengths
        .iter()
        .filter(|x| **x == *min_length)
        .count();

    println!("Part 2: {}", result2);
}
