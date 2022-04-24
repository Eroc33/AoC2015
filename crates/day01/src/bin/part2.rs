use std::io::BufRead;

use shared::bail;

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

#[cfg(test)]
#[test]
fn day01_part2_example() {
    shared::check_example(solution, todo!(), todo!())
}
