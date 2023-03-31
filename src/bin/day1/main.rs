use std::process;

use clap::{command, Arg};

fn parse_input(input: &str) -> Result<Vec<i32>, std::io::Error> {
    std::fs::read_to_string(input)?
        .trim()
        .chars()
        .map(|c| match c {
            '(' => Ok(1),
            ')' => Ok(-1),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid input",
            )),
        })
        .collect()
}

fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn part2(input: &[i32]) -> Option<usize> {
    let mut floor = 0;
    for (i, &c) in input.iter().enumerate() {
        floor += c;
        if floor == -1 {
            return Some(i + 1);
        }
    }

    None
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let input_data = match parse_input(&input_file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            process::exit(1);
        }
    };

    let result1 = part1(&input_data);
    println!("Part 1: {}", result1);

    let result2 = part2(&input_data);
    match result2 {
        Some(r) => println!("Part 2: {}", r),
        None => println!("Part 2: Not found"),
    }
}
