use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

use crate::{Action, Instruction, Point};

fn parse_action(input: &str) -> IResult<&str, Action> {
    alt((
        map(tag("turn on"), |_| Action::TurnOn),
        map(tag("turn off"), |_| Action::TurnOff),
        map(tag("toggle"), |_| Action::Toggle),
    ))(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            nom::character::complete::u32,
            char(','),
            nom::character::complete::u32,
        ),
        |(x, y)| Point { x, y },
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            parse_action,
            preceded(tag(" "), parse_point),
            preceded(tag(" through "), parse_point),
        )),
        |(action, from, to)| Instruction { action, from, to },
    )(input)
}

impl FromStr for Instruction {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        all_consuming(parse_instruction)(s)
            .map_err(|e| e.to_owned())
            .finish()
            .map(|(_, instruction)| instruction)
    }
}
