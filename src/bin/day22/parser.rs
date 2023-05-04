use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish,
};

use crate::Boss;

fn parse_boss(input: &str) -> nom::IResult<&str, Boss> {
    map(
        separated_pair(
            preceded(tag("Hit Points: "), nom::character::complete::i32),
            line_ending,
            preceded(tag("Damage: "), nom::character::complete::i32),
        ),
        |(hp, damage)| Boss { hp, damage },
    )(input)
}

impl FromStr for Boss {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        all_consuming(parse_boss)(s)
            .map_err(|e| e.to_owned())
            .finish()
            .map(|(_, boss)| boss)
    }
}
