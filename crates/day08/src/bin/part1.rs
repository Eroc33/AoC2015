use std::io::BufRead;

use shared::{combine::{*, parser::{char::hex_digit}}, anyhow, parameterized_tests};

pub fn escape_parser<Input>() -> impl Parser<Input, Output = u8>
where
    Input: Stream<Token = char>,
{
    token('\\').with(
    choice((
        token('\\').map(|_| b'\\'),
        token('"').map(|_| b'"'),
        (token('x'),hex_digit(),hex_digit()).map(|(_,hi,lo)| (hi.to_digit(16).expect("checked by parser") << 4 | lo.to_digit(16).expect("checked by parser") ) as u8),
    )))
}
pub fn parser<Input>() -> impl Parser<Input, Output = Vec<u8>>
where
    Input: Stream<Token = char>,
{
    between(token('"'),token('"'),many(
        choice((
            attempt(escape_parser()),
            satisfy(|c| c != '"').map(|c| c as u8),
        ))
    ))
}

fn solution(input: impl BufRead) -> shared::Result<usize> {
    let mut total = 0;
    for line in input.lines(){
        let line = line?;
        let (parsed, rest) = parser()
            .easy_parse(shared::combine::stream::position::Stream::new(&*line))
            .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
        assert!(rest.input.len() == 0);
        total += line.len() - parsed.len()
    }

    Ok(total)
}

shared::main!(solution);

parameterized_tests! {
    day08_part1_example_a: solution(r#""""#) == 2,
    day08_part1_example_b: solution(r#""abc""#) == 2,
    day08_part1_example_c: solution(r#""aaa\"aaa""#) == 3,
    day08_part1_example_d: solution(r#""\x27""#) == 5,

    day08_part1_example_e: solution(
r#"""
"abc"
"aaa\"aaa"
"\x27""#) == 12,
}