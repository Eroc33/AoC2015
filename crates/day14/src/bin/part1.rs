use std::{io::BufRead};
use shared::{combine::{*, parser::char::string}, anyhow};

pub struct Reindeer{
    name: String,
    speed: usize,
    time: usize,
    rest_time: usize,
}

pub fn parser<Input>() -> impl Parser<Input, Output = Vec<Reindeer>>
where
    Input: Stream<Token = char>,
{
    sep_by1((
        many1(satisfy(|c| c != ' ')),
        string(" can fly "),
        shared::parse::usize(),
        string(" km/s for "),
        shared::parse::usize(),
        string(" seconds, but then must rest for "),
        shared::parse::usize(),
        string(" seconds."),
    ).map(|(name,_,speed,_,time,_,rest_time,_)| Reindeer{name,speed,time,rest_time}),shared::parse::lax_newline())
}

fn evaluate_distance(reindeer: Reindeer, time: usize) -> usize
{
    let full_cycle_time = reindeer.time + reindeer.rest_time;
    let full_cycle_distance = reindeer.time*reindeer.speed;
    let completed_cycles = time/full_cycle_time;

    let current_cycle_time = time - (full_cycle_time*completed_cycles);

    let partial_cycle_distance = reindeer.time.min(current_cycle_time) * reindeer.speed;

    completed_cycles * full_cycle_distance + partial_cycle_distance
}

fn evaluate_race(mut input: impl BufRead, time: usize) -> shared::Result<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (reindeer, rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    Ok(
        reindeer.into_iter()
        .map(|reindeer| evaluate_distance(reindeer, time))
        .max()
        .expect("No best reindeer found")
    )

}

fn solution(mut input: impl BufRead) -> shared::Result<usize> {
    evaluate_race(input, 2503)
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day14_part1_example() {
    shared::check_example(|input| evaluate_race(input,1000), 
r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#, 1120)
}
