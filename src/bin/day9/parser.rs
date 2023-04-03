use std::str::FromStr;

use nom::{bytes::complete::tag, combinator::all_consuming, sequence::preceded, Finish};

use crate::Distance;

fn parse_distance(input: &str) -> Result<(String, String, u32), nom::error::Error<String>> {
    // London to Dublin = 464
    all_consuming(nom::sequence::tuple((
        nom::character::complete::alpha1,
        preceded(tag(" to "), nom::character::complete::alpha1),
        preceded(tag(" = "), nom::character::complete::u32),
    )))(input)
    .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
    .finish()
    .map(|(_, (from, to, distance))| (from.to_string(), to.to_string(), distance))
}

impl FromStr for Distance {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to, distance) = parse_distance(s)?;
        Ok(Distance { from, to, distance })
    }
}
