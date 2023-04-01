use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map, verify},
    multi::separated_list0,
    Finish, IResult,
};

use crate::Present;

fn parse_present(input: &str) -> IResult<&str, Present> {
    all_consuming(map(
        verify(
            separated_list0(tag("x"), nom::character::complete::i32),
            |list: &Vec<i32>| list.len() == 3,
        ),
        |list| {
            let [l, w, h] = list[..] else { panic!("list.len() != 3") };
            Present {
                length: l,
                width: w,
                height: h,
            }
        },
    ))(input)
}

impl FromStr for Present {
    type Err = nom::error::Error<String>;

    fn from_str(input: &str) -> Result<Present, Self::Err> {
        parse_present(input)
            .map_err(|e| e.to_owned())
            .finish()
            .map(|(_, present)| present)
    }
}
