use std::{io::BufRead, collections::HashMap};
use shared::{combine::{*, parser::char::string}, anyhow};

#[derive(Debug)]
pub struct Reindeer{
    name: String,
    speed: usize,
    time: usize,
    rest_time: usize,
}

impl Reindeer{
    fn resting_at(&self, time: usize) -> bool{
        let full_cycle_time = self.time + self.rest_time;
        let completed_cycles = time/full_cycle_time;
        let current_cycle_time = time - (full_cycle_time*completed_cycles);

        current_cycle_time >= self.time
    }
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

fn evaluate_race(mut input: impl BufRead, total_time: usize) -> shared::Result<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (reindeer, rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    let mut distances = HashMap::new();
    let mut scores = HashMap::new();

    for time in 0..total_time{
        for reindeer in &reindeer{
            if !reindeer.resting_at(time){
                *distances.entry(&reindeer.name).or_insert(0) +=  reindeer.speed
            }
        }
        let best_distance = distances.iter().map(|(_k,v)| *v).max().ok_or_else(|| anyhow!("No winning reindeer"))?;
        for reindeer in &reindeer{
            if distances.get(&reindeer.name) == Some(&best_distance){
                *scores.entry(&reindeer.name).or_insert(0) += 1;
            }
        }
    }

    dbg!(&reindeer,&distances,&scores);

    Ok(
        scores.into_iter()
        .map(|(_k,v)| v)
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
fn day14_part2_example() {
    shared::check_example(|input| evaluate_race(input,1000), 
r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#, 689)
}
