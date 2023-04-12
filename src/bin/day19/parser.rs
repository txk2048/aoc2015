use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

use crate::Replacement;

fn parse_replacement(s: &str) -> IResult<&str, Replacement> {
    map(
        separated_pair(alpha1, tag(" => "), alpha1),
        |(from, to): (&str, &str)| Replacement {
            from: from.to_string(),
            to: to.to_string(),
        },
    )(s)
}

fn parse_replacements(s: &str) -> IResult<&str, Vec<Replacement>> {
    separated_list1(line_ending, parse_replacement)(s)
}

fn parse_molecule(s: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(s)
}

pub(crate) fn parse_input(s: &str) -> Result<(Vec<Replacement>, String), Error<String>> {
    all_consuming(separated_pair(
        terminated(parse_replacements, line_ending),
        line_ending,
        parse_molecule,
    ))(s)
    .map_err(|e: nom::Err<Error<&str>>| e.to_owned())
    .finish()
    .map(|(_, (rules, molecule))| (rules, molecule))
}
