use std::io::BufRead;

use shared::{combine::Parser, parameterized_tests};

use day02::*;

fn required_paper(prism: Prism) -> u64 {
    prism.surface_area() + prism.smallest_side_surface_area()
}

fn solution(input: impl BufRead) -> shared::Result<u64> {
    let mut boxes = vec![];
    for line in input.lines() {
        let line = line?;
        let item = parser().parse(&line)?.0;
        boxes.push(item);
    }
    Ok(boxes.into_iter().map(required_paper).sum())
}

shared::main!(solution);

parameterized_tests! {
    day02_part1_example_a: solution("2x3x4") == 58,
    day02_part1_example_b: solution("1x1x10") == 43,
}
