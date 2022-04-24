use std::io::BufRead;

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

#[cfg(test)]
#[test]
fn day01_part1_example() {
    shared::check_example(solution, todo!(), todo!())
}
