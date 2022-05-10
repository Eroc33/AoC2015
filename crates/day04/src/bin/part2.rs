use std::io::BufRead;

use md5::{Digest, Md5};
use shared::{bail};

fn solution(mut input: impl BufRead) -> shared::Result<u32> {
    let mut key = String::new();
    input.read_to_string(&mut key)?;

    for i in 0.. {
        let result = Md5::digest(format!("{key}{i}"));
        let result = format!("{:02x}{:02x}{:02x}", result[0], result[1], result[2]);
        if result.starts_with("000000") {
            return Ok(i);
        }
    }

    bail!("HashCash not found");
}

shared::main!(solution);
