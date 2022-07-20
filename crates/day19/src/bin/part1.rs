use std::collections::HashSet;
use std::io::BufRead;
use std::ops::Range;

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
    let ((replacements, base), rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    let mut set = HashSet::new();
    
    for (from,to) in replacements{
        for molecule in each_replacement(&base,&from,&to){
            set.insert(molecule);
        }
    }
    Ok(set.len())
}

fn each_replacement<'a>(base: &'a str, from: &'a str, to: &'a str) -> impl Iterator<Item=String> + 'a{
    all_matching_positions(base,from).map(|range|{
        let mut str = base.to_string();
        str.replace_range(range, to);
        str
    })
}

fn all_matching_positions<'a>(base: &'a str, pattern: &'a str) -> impl Iterator<Item=Range<usize>> + 'a{
    base.char_indices().filter_map(move |(i,_)|{
        if base[i..].starts_with(pattern){
            Some(i..i+pattern.len())
        }else{
            None
        }
    })
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day19_part1_example() {
    shared::check_example(
        solution,
        "H => HO
H => OH
O => HH

HOH",
        4,
    )
}
