use std::io::BufRead;

use shared::{bail, parameterized_tests};

fn solution(input: impl BufRead) -> shared::Result<i64> {
    let mut floor = 0i64;
    let mut pos = 1;
    for byte in input.bytes() {
        let byte = byte?;
        match byte {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            return Ok(pos);
        }
        pos += 1;
    }
    bail!("Never reached basement")
}

shared::main!(solution);

parameterized_tests! {
    day01_part2_example_a: solution(")") == 1,
    day01_part2_example_b: solution("()())") == 5,
}
