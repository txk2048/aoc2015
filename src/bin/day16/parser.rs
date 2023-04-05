use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    Finish,
};

use crate::Aunt;

fn parse_key(s: &str) -> nom::IResult<&str, &str> {
    alt((
        tag("children"),
        tag("cats"),
        tag("samoyeds"),
        tag("pomeranians"),
        tag("akitas"),
        tag("vizslas"),
        tag("goldfish"),
        tag("trees"),
        tag("cars"),
        tag("perfumes"),
    ))(s)
}

fn parse_attribute(s: &str) -> nom::IResult<&str, (&str, u32)> {
    separated_pair(parse_key, tag(": "), u32)(s)
}

impl FromStr for Aunt {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        all_consuming(map(
            tuple((
                preceded(tag("Sue "), u32),
                preceded(tag(": "), separated_list1(tag(", "), parse_attribute)),
            )),
            |(id, attributes)| Aunt {
                id,
                attributes: attributes
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            },
        ))(s)
        .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
        .finish()
        .map(|(_, aunt)| aunt)
    }
}
