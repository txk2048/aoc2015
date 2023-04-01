mod parser;

use std::{error::Error, process};

use clap::{command, Arg};

struct Present {
    length: i32,
    width: i32,
    height: i32,
}

fn parse_input(input: &str) -> Result<Vec<Present>, Box<dyn Error>> {
    let presents = std::fs::read_to_string(input)?
        .lines()
        .map(|line| line.parse::<Present>())
        .collect::<Result<Vec<Present>, _>>()?;

    Ok(presents)
}

fn part1(presents: &[Present]) -> i32 {
    presents
        .iter()
        .map(|present| {
            let [l, w, h] = [present.length, present.width, present.height];
            let surface_area = 2 * l * w + 2 * l * h + 2 * w * h;
            let areas = [l * w, w * h, h * l];

            surface_area + areas.iter().min().unwrap()
        })
        .sum()
}

fn part2(presents: &[Present]) -> i32 {
    presents
        .iter()
        .map(|present| {
            let [l, w, h] = [present.length, present.width, present.height];
            let [a, b, c] = [l + l + w + w, w + w + h + h, h + h + l + l];
            let bow = l * w * h;
            a.min(b).min(c) + bow
        })
        .sum()
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let input_data = match parse_input(input_file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing input: {}", e.to_string());
            process::exit(1);
        }
    };

    println!("Part 1: {}", part1(&input_data));
    println!("Part 2: {}", part2(&input_data));
}
