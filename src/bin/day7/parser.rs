use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

use crate::{Instruction, Operand, Operation};

fn parse_wire(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(input)
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(nom::character::complete::u16, |v: u16| Operand::Value(v)),
        map(parse_wire, |s: String| Operand::Wire(s)),
    ))(input)
}

fn parse_assign(input: &str) -> IResult<&str, Instruction> {
    // sample input: "123 -> x"
    // sample input: "x -> d"

    map(
        separated_pair(parse_operand, tag(" -> "), parse_wire),
        |(op, out)| Instruction {
            op: Operation::Assign,
            arg1: op,
            arg2: None,
            output: out,
        },
    )(input)
}

fn parse_not(input: &str) -> IResult<&str, Instruction> {
    // sample input: "NOT x -> h"

    map(
        preceded(
            tag("NOT "),
            separated_pair(parse_operand, tag(" -> "), parse_wire),
        ),
        |(op, out)| Instruction {
            op: Operation::Not,
            arg1: op,
            arg2: None,
            output: out,
        },
    )(input)
}

fn parse_two_operand(input: &str) -> IResult<&str, Instruction> {
    // sample input: "x AND y -> d"
    // sample input: "x OR y -> d"
    // sample input: "x LSHIFT 2 -> f"
    // sample input: "y RSHIFT 2 -> g"

    map(
        tuple((
            parse_operand,
            preceded(
                char(' '),
                alt((tag("AND"), tag("OR"), tag("LSHIFT"), tag("RSHIFT"))),
            ),
            preceded(char(' '), parse_operand),
            preceded(tag(" -> "), parse_wire),
        )),
        |(op1, operation, op2, out)| Instruction {
            op: match operation {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "LSHIFT" => Operation::LShift,
                "RSHIFT" => Operation::RShift,
                _ => unreachable!(),
            },
            arg1: op1,
            arg2: Some(op2),
            output: out,
        },
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    // sample input: "123 -> x"
    // sample input: "x AND y -> d"
    // sample input: "x OR y -> d"
    // sample input: "x LSHIFT 2 -> f"
    // sample input: "y RSHIFT 2 -> g"
    // sample input: "NOT x -> h"

    alt((parse_assign, parse_not, parse_two_operand))(input)
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
