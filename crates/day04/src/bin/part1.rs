use std::io::BufRead;

use md5::{Digest, Md5};
use shared::{bail, parameterized_tests};

fn solution(mut input: impl BufRead) -> shared::Result<u32> {
    let mut key = String::new();
    input.read_to_string(&mut key)?;

    for i in 0.. {
        let result = Md5::digest(format!("{key}{i}"));
        let result = format!("{:02x}{:02x}{:02x}", result[0], result[1], result[2]);
        if result.starts_with("00000") {
            return Ok(i);
        }
    }

    bail!("HashCash not found");
}

shared::main!(solution);

parameterized_tests! {
    day04_part1_example_a: solution("abcdef") == 609043,
    day04_part1_example_b: solution("pqrstuv") == 1048970,
}
