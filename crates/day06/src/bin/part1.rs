use shared::{combine::{EasyParser}, parameterized_tests, anyhow};
use std::{io::BufRead};

use day06::{Command, Action};

fn solution(input: impl BufRead) -> shared::Result<usize> {
    let mut commands = vec![];
    for line in input.lines(){
        let line = line?;
        let (parsed, _rest) = Command::parser()
            .easy_parse(shared::combine::stream::position::Stream::new(&*line))
            .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
        commands.push(parsed.clone());
    }

    let mut lights = vec![false;1000*1000];

    for command in commands{
        for y in command.y_range{
            for x in command.x_range.clone(){
                lights[x+(y*1000)] = match command.action {
                    Action::On => true,
                    Action::Off => false,
                    Action::Toggle => !lights[x+(y*1000)]
                };
            }
        }
    }

    Ok(lights.into_iter().filter(|state| *state).count())
}

shared::main!(solution);

parameterized_tests! {
    day06_part1_example_a: solution("turn on 0,0 through 999,999") == 1000000,
    day06_part1_example_b: solution("toggle 0,0 through 999,0") == 1000,
    day06_part1_example_c: solution("turn off 499,499 through 500,500") == 0,
}