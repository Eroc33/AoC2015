use std::io::BufRead;

use day07::*;
use shared::{combine::EasyParser, anyhow};

fn solution(mut input: impl BufRead) -> shared::Result<u16> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (mut instructions, _rest) = Instructions::parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;

    let value = WireName::from("a").evaluate(&mut EvaluationContext::new(), &instructions).expect("No value on wire a!");

    instructions.override_wire(WireName::from("b"), value);

    let value = WireName::from("a").evaluate(&mut EvaluationContext::new(), &instructions).expect("No value on wire a!");

    Ok(value)
}

shared::main!(solution);