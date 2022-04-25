use std::{collections::HashMap, io::BufRead};

use day03::{parser, Direction};
use shared::{combine::Parser, parameterized_tests};

fn solution(mut input: impl BufRead) -> shared::Result<usize> {
    let mut string = String::new();
    input.read_to_string(&mut string)?;
    let (directions, _) = parser().parse(&string)?;
    let mut visits: HashMap<Direction, usize> = HashMap::new();
    let mut pos = [0, 0];

    *visits.entry(pos).or_default() += 1;

    for dir in directions {
        pos[0] += dir[0];
        pos[1] += dir[1];
        *visits.entry(pos).or_default() += 1;
    }

    let one_visit_count = visits.into_iter().filter(|(_, count)| *count >= 1).count();

    Ok(one_visit_count)
}

shared::main!(solution);

parameterized_tests! {
    day03_part1_example_a: solution(">") == 2,
    day03_part1_example_b: solution("^>v<") == 4,
    day03_part1_example_c: solution("^v^v^v^v^v") == 2,
}
