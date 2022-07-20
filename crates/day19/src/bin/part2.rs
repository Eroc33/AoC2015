use std::io::BufRead;

use rand::SeedableRng;
use rand::prelude::SliceRandom;
use shared::anyhow;
use shared::combine::optional;
use shared::combine::{
    many1,
    parser::{char::string, repeat::repeat_until},
    satisfy, EasyParser, Parser, Stream,
};

pub fn parser<Input>() -> impl Parser<Input, Output = (Vec<(String, String)>, String)>
where
    Input: Stream<Token = char>,
{
    let replacement_parser = (
        many1(satisfy(|c: char| !c.is_whitespace())).skip(string(" => ")),
        many1(satisfy(|c: char| !c.is_whitespace())),
    );
    (
        repeat_until(
            replacement_parser.skip(shared::parse::lax_newline()),
            shared::parse::lax_newline(),
        ).skip(shared::parse::lax_newline()),
        many1(satisfy(|c: char| !c.is_whitespace())).skip(optional(shared::parse::lax_newline())),
    )
}

fn solution(mut input: impl BufRead) -> shared::Result<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let ((replacements, target), rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    let mut reverse_replacements: Vec<_> = replacements.into_iter().map(|(from,to)| (to, from)).collect();

    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut molecule = target.to_string();
    let mut steps = 0;

    loop{
        molecule = target.to_string();
        steps = 0;
        let mut reset = true;
        while reset{
            reset = false;
            for (from,to) in &reverse_replacements {
                while molecule.contains(from)
                {
                    molecule = molecule.replacen(from, to,1);
                    steps += 1;
                    reset = true;
                }
            }
        }
        if molecule == "e"{
            break;
        }
        reverse_replacements.shuffle(&mut rng)
    }
    
    Ok(steps)
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day19_part2_example_a() {
    shared::check_example(
        solution,
        "e => H
e => O
H => HO
H => OH
O => HH

HOH",
        3,
    )
}

#[cfg(test)]
#[test]
fn day19_part2_example_b() {
    shared::check_example(
        solution,
        "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO",
        6,
    )
}
