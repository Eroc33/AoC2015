use shared::{combine::{EasyParser}, parameterized_tests, anyhow};
use std::{io::BufRead};

use day06::{Command, Action};

fn solution(input: impl BufRead) -> shared::Result<u64> {
    let mut commands = vec![];
    for line in input.lines(){
        let line = line?;
        let (parsed, _rest) = Command::parser()
            .easy_parse(shared::combine::stream::position::Stream::new(&*line))
            .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
        commands.push(parsed.clone());
    }

    let mut lights = vec![0u64;1000*1000];

    for command in commands{
        for y in command.y_range{
            for x in command.x_range.clone(){
                lights[x+(y*1000)] = match command.action {
                    Action::On => lights[x+(y*1000)] + 1,
                    Action::Off => lights[x+(y*1000)].saturating_sub(1),
                    Action::Toggle => lights[x+(y*1000)] + 2,
                };
            }
        }
    }

    Ok(lights.into_iter().sum())
}

shared::main!(solution);

parameterized_tests! {
    day06_part2_example_a: solution("turn on 0,0 through 0,0") == 1,
    day06_part2_example_b: solution("toggle 0,0 through 999,999") == 2000000,
}