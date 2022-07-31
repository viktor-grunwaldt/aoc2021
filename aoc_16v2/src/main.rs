use itertools::Itertools;
use std::{fs::read_to_string};
use nom::{
    bits::complete::tag,
    branch::alt,
    combinator::{map, flat_map},
    complete::take,
    multi::{many0, length_count},
    sequence::{pair, preceded},
    IResult, InputLength, Parser, 
};

#[derive(Debug, Clone)]
struct Packet {
    type_id: u8,
    value: PacVals,
    version: u8,
}

#[derive(Debug, Clone)]
enum PacVals {
    Literal(u64),
    OperatorPacket(Vec<Packet>)
}

type BitInput<'a> = (&'a [u8], usize);

fn p_leading_chunk(input: BitInput) -> IResult<BitInput, u8> {
    preceded(tag(1, 1usize), take(4u8))(input)
}
fn p_tail_chunk(input: BitInput) -> IResult<BitInput, u8> {
    preceded(tag(0, 1usize), take(4u8))(input)
}

fn nibbles_to_u64(input: (Vec<u8>, u8)) -> u64 {
    input.0
        .into_iter()
        .chain(std::iter::once(input.1))
        .fold(0, |acc, x| (acc << 4) + x as u64)
}

fn p_literal(input: BitInput) -> IResult<BitInput, PacVals> {
    let parser = map(
        pair(many0(p_leading_chunk), p_tail_chunk),
        nibbles_to_u64
    );
    map(parser, PacVals::Literal)(input)
}

fn p_operator(input: BitInput) -> IResult<BitInput, PacVals> {
    map(
        alt((
            p_operator_by_bitlen,
            p_operator_by_pacnum
        )), 
        PacVals::OperatorPacket
    )(input)
}

fn p_operator_by_pacnum(input: BitInput) -> IResult<BitInput, Vec<Packet>> {
    preceded(
        tag(1, 1u8),
        length_count(
            take::<_,u16,usize,_>(11), 
            p_packet
    ))(input)
}

/// this spicy meatball won't work because
/// length_value isn't implemented on bits
/// see: https://github.com/Geal/nom/issues/1477 and 1478
//
// fn p_operator_by_bitlen(input: BitInput) -> IResult<BitInput, Vec<Packet>> {
//     preceded(
//         tag(0b1, 1u8),
//         length_value(
//             take::<_,u16,usize,_>(15), 
//             many0(p_packet)
//     ))(input)
// }

fn scuffed_length_value<'a>(n:usize) 
-> impl Parser<BitInput<'a>, 
                Vec<Packet>, 
                nom::error::Error<BitInput<'a>>> 
{
    move |mut input: BitInput<'a>| {
        let mut packets = Vec::new();
        let input_len = input.input_len();

        while input_len - input.input_len() < n {
            let (new_input, next_packet) = p_packet(input)?;
            packets.push(next_packet);
            input = new_input;
        }

        match (input_len - input.input_len()).cmp(&n) {
            std::cmp::Ordering::Equal => Ok((input, packets)),
            std::cmp::Ordering::Greater => panic!("nom error handling hard"),
            // std::cmp::Ordering::Greater => Err(nom::Err::Failure(input)),
            std::cmp::Ordering::Less => unreachable!(),
        }
    }
}

fn p_operator_by_bitlen(input: BitInput) -> IResult<BitInput, Vec<Packet>> {
    preceded(
        tag(0, 1u8),
        flat_map(
            take(15usize), 
            scuffed_length_value,
        ),
    )(input)
}


fn p_packet(
    input: BitInput,
) -> IResult<BitInput, Packet> {
    let (input, version) = take::<_,u8,usize,_>(3)(input)?;
    let (input, type_id) = take::<_,u8,usize,_>(3)(input)?;

    let (input, packet) = match type_id {
        4 => p_literal(input)?,
        _ => p_operator(input)?,
    };
    Ok((
        input,
        Packet {
            version: version as u8,
            type_id: type_id as u8,
            value: packet,
        },
    ))
}

fn p_input(input: &str) -> Packet {
    let byte_input: Vec<u8> = input
        .chars()
        .chunks(2)
        .into_iter()
        .map(|a| a.into_iter().join(""))
        .map(|s| u8::from_str_radix(&s, 16)
                    .expect("Couldn't parse hex"))
        .collect();

    p_packet((&byte_input, 0))
        .expect("invalid input")
        .1
}

fn sum_versions(p: Packet) -> u32 {
    p.version as u32 + 
    match p.value {
        PacVals::Literal(_) 
            => 0,
        PacVals::OperatorPacket(v) 
            => v.into_iter().map(sum_versions).sum()
    }
}

fn eval_op(v:Vec<Packet>, op: u8) -> u64{
    let mut i = v.into_iter().map(eval);
    match op {
        // 4 => parse_literal(q),
        0 => i.sum(),
        1 => i.product(),
        2 => i.min().unwrap(),
        3 => i.max().unwrap(),
        5 => (i.next().unwrap() > i.next().unwrap()) as u64,
        6 => (i.next().unwrap() < i.next().unwrap()) as u64,
        7 => (i.next().unwrap() == i.next().unwrap()) as u64,
        _ => unreachable!()
    }
}

fn eval(p:Packet) -> u64 {
    match p.value {
        PacVals::Literal(l) => l,
        PacVals::OperatorPacket(v) => eval_op(v, p.type_id)
    }
}

fn parts(name: &str) -> (u32, u64) {
    let input = read_to_string(name)
                .expect("Couldn't read file");
    let p = p_input(&input);
    (sum_versions(p.clone()), eval(p))
}

fn main() {
    println!("Hello, world!");
    let (pt1, pt2) = parts("input.txt");
    println!("pt 1: {}", pt1);    
    println!("pt 2: {}", pt2);    
}
