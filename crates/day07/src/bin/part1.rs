use std::io::BufRead;

use day07::*;
use shared::{combine::EasyParser, anyhow};

fn solution(mut input: impl BufRead) -> shared::Result<u16> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (instructions, _rest) = Instructions::parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;

    let value = WireName::from("a").evaluate(&mut EvaluationContext::new(), &instructions).expect("No value on wire a!");

    Ok(value)
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day07_part1_example() {
    use std::collections::HashMap;

    fn evaluate_all_wires(mut input: impl BufRead) -> shared::Result<HashMap<WireName,u16>>{
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        let (instructions, _rest) = Instructions::parser()
            .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
            .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;

        let mut values = HashMap::new();
        let mut evaluation_context = EvaluationContext::new();
        for wirename in instructions.wirenames(){
            if let Some(val) = wirename.evaluate(&mut evaluation_context, &instructions){
                values.insert(wirename.clone(), val);
            }
        }

        Ok(values)
    }


    shared::check_example(evaluate_all_wires, "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i", 
[
    ("d", 72),
    ("e", 507),
    ("f", 492),
    ("g", 114),
    ("h", 65412),
    ("i", 65079),
    ("x", 123),
    ("y", 456),
    ].into_iter().map(|(k,v)| (WireName::from(k), v)).collect())
}
