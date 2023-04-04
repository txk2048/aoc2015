use std::str::FromStr;

use clap::{command, Arg};

#[derive(Debug, Clone, Copy)]
enum ReindeerState {
    Flying,
    Resting,
}

#[derive(Debug, Clone)]
struct Reindeer {
    speed: i32,
    fly_time: i32,
    rest_time: i32,

    distance: i32,
    points: i32,

    state: ReindeerState,
    state_time: i32,
}

impl Reindeer {
    fn tick(&mut self) {
        match self.state {
            ReindeerState::Flying => {
                self.distance += self.speed;
                self.state_time -= 1;
                if self.state_time <= 0 {
                    self.state = ReindeerState::Resting;
                    self.state_time = self.rest_time;
                }
            }

            ReindeerState::Resting => {
                self.state_time -= 1;
                if self.state_time <= 0 {
                    self.state = ReindeerState::Flying;
                    self.state_time = self.fly_time;
                }
            }
        }
    }
}

impl FromStr for Reindeer {
    type Err = nom::error::Error<String>;

    fn from_str<'a>(input: &'a str) -> Result<Self, Self::Err> {
        use nom::{
            bytes::complete::tag,
            character::complete::{alpha1, i32},
            combinator::all_consuming,
            Finish,
        };

        let parser = |input: &'a str| {
            let (input, _) = alpha1(input)?;
            let (input, _) = tag(" can fly ")(input)?;
            let (input, speed) = i32(input)?;
            let (input, _) = tag(" km/s for ")(input)?;
            let (input, fly_time) = i32(input)?;
            let (input, _) = tag(" seconds, but then must rest for ")(input)?;
            let (input, rest_time) = i32(input)?;
            let (input, _) = tag(" seconds.")(input)?;

            Ok((
                input,
                Reindeer {
                    speed,
                    fly_time,
                    rest_time,
                    distance: 0,
                    points: 0,
                    state: ReindeerState::Flying,
                    state_time: fly_time,
                },
            ))
        };

        all_consuming(parser)(input)
            .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
            .finish()
            .map(|(_, v)| v)
    }
}

fn part1(mut reindeer: Vec<Reindeer>) -> i32 {
    for _ in 0..2503 {
        reindeer.iter_mut().for_each(|r| r.tick());
    }

    reindeer.iter().map(|r| r.distance).max().unwrap()
}

fn part2(mut reindeer: Vec<Reindeer>) -> i32 {
    for _ in 0..2503 {
        reindeer.iter_mut().for_each(|r| r.tick());

        let max_distance = reindeer.iter().map(|r| r.distance).max().unwrap();
        reindeer
            .iter_mut()
            .filter(|r| r.distance == max_distance)
            .for_each(|r| r.points += 1);
    }

    reindeer.iter().map(|r| r.points).max().unwrap()
}

fn main() {
    let matches = command!()
        .arg(Arg::new("FILE").help("Input file").required(true))
        .get_matches();

    let input_file = matches
        .get_one::<String>("FILE")
        .expect("Error getting input file");

    let reindeer = std::fs::read_to_string(input_file)
        .expect("Error reading input file")
        .lines()
        .map(|line| line.parse::<Reindeer>().expect("Error parsing reindeer"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(reindeer.clone()));
    println!("Part 2: {}", part2(reindeer));
}
