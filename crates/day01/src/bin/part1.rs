use std::io::BufRead;

use shared::parameterized_tests;

fn solution(input: impl BufRead) -> shared::Result<i64> {
    let mut floor = 0i64;
    for byte in input.bytes() {
        let byte = byte?;
        match byte {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        }
    }
    Ok(floor)
}

shared::main!(solution);

parameterized_tests! {
    day01_part1_example_a: solution("(())") == 0,
    day01_part1_example_b: solution("()()") == 0,
    day01_part1_example_c: solution("(((") == 3,
    day01_part1_example_d: solution("(()(()(") == 3,
    day01_part1_example_e: solution("))(((((") == 3,
    day01_part1_example_f: solution("())") == -1,
    day01_part1_example_g: solution("))(") == -1,
    day01_part1_example_h: solution(")))") == -3,
    day01_part1_example_i: solution(")())())") == -3,
}
