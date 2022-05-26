use std::io::BufRead;

use shared::parameterized_tests;

fn encode(input: &str) -> String{
    vec!['"'].into_iter().chain(input.chars().flat_map(|char|{
        match char{
            '\\' => vec!['\\','\\'],
            '"' => vec!['\\','"'],
            c if c.is_ascii() => vec![c],
            other => {
                panic!("Can't encode char: {other}")
            }
        }
    })).chain(vec!['"']).collect()
}

fn solution(input: impl BufRead) -> shared::Result<usize> {
    let mut total = 0;
    for line in input.lines(){
        let line = line?;
        let encoded = encode(&line);
        total += encoded.len() - line.len()
    }

    Ok(total)
}

shared::main!(solution);

parameterized_tests! {
    day08_part2_example_a: solution(r#""""#) == 4,
    day08_part2_example_b: solution(r#""abc""#) == 4,
    day08_part2_example_c: solution(r#""aaa\"aaa""#) == 6,
    day08_part2_example_d: solution(r#""\x27""#) == 5,

    day08_part2_example_e: solution(
r#"""
"abc"
"aaa\"aaa"
"\x27""#) == 19,
}