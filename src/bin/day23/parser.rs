use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish,
};

use crate::{Instruction, TargetRegister};

fn parse_register(s: &str) -> nom::IResult<&str, TargetRegister> {
    map(one_of("ab"), |c| match c {
        'a' => TargetRegister::A,
        'b' => TargetRegister::B,
        _ => unreachable!(),
    })(s)
}

fn parse_offset(s: &str) -> nom::IResult<&str, i32> {
    nom::character::complete::i32(s)
}

fn parse_instruction(s: &str) -> nom::IResult<&str, Instruction> {
    let parse_half = map(preceded(tag("hlf "), parse_register), |register| {
        Instruction::Half(register)
    });

    let parse_triple = map(preceded(tag("tpl "), parse_register), |register| {
        Instruction::Triple(register)
    });

    let parse_increment = map(preceded(tag("inc "), parse_register), |register| {
        Instruction::Increment(register)
    });

    let parse_jump = map(preceded(tag("jmp "), parse_offset), |offset| {
        Instruction::Jump(offset)
    });

    let parse_jump_if_even = map(
        preceded(
            tag("jie "),
            separated_pair(parse_register, tag(", "), parse_offset),
        ),
        |(register, offset)| Instruction::JumpIfEven(register, offset),
    );

    let parse_jump_if_one = map(
        preceded(
            tag("jio "),
            separated_pair(parse_register, tag(", "), parse_offset),
        ),
        |(register, offset)| Instruction::JumpIfOne(register, offset),
    );

    alt((
        parse_half,
        parse_triple,
        parse_increment,
        parse_jump,
        parse_jump_if_even,
        parse_jump_if_one,
    ))(s)
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
