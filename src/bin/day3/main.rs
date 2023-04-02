use std::collections::HashSet;

use clap::{command, Arg};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_direction(&mut self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }

        *self
    }
}

fn parse_input(input_file: &str) -> Result<Vec<Direction>, String> {
    let input = std::fs::read_to_string(input_file).map_err(|e| e.to_string())?;

    let directions = input
        .trim()
        .chars()
        .map(|c| match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", c)),
        })
        .collect::<Result<Vec<Direction>, String>>()?;

    Ok(directions)
}

fn part1(directions: &[Direction]) -> usize {
    let mut houses = HashSet::new();
    let mut santa = Point::new(0, 0);

    houses.insert(santa);

    for direction in directions {
        santa.move_direction(direction);
        houses.insert(santa);
    }

    houses.len()
}

fn part2(directions: &[Direction]) -> usize {
    let mut houses = HashSet::new();

    // stanta x, y
    let mut santa = Point::new(0, 0);

    // robo santa rx, ry
    let mut robo_santa = Point::new(0, 0);

    houses.insert(santa);

    for (i, direction) in directions.iter().enumerate() {
        let rturn = i % 2 == 0;
        if rturn {
            santa.move_direction(direction);
            houses.insert(santa);
        } else {
            robo_santa.move_direction(direction);
            houses.insert(robo_santa);
        }
    }

    houses.len()
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let directions = parse_input(&input_file).expect("Error parsing input");

    println!("Part 1: {}", part1(&directions));
    println!("Part 2: {}", part2(&directions));
}
