use std::io::BufRead;

use shared::parameterized_tests;

fn solution(input: impl BufRead) -> shared::Result<i64> {
    let value: serde_json::Value = serde_json::from_reader(input)?;
    
    let mut nums = vec![];
    let mut open = vec![value];
    while let Some(val) = open.pop(){
        match val{
            serde_json::Value::Null => {},
            serde_json::Value::Bool(_) => {},
            serde_json::Value::Number(num) => {
                nums.push(num.as_i64().expect("Non-i64 number"));
            }
            serde_json::Value::String(_) => {},
            serde_json::Value::Array(array) => {
                open.extend(array);
            }
            serde_json::Value::Object(object) => {
                open.extend(object.into_iter().map(|(_k,v)| v));
            }
        }
    }
    Ok(nums.into_iter().sum())
}

shared::main!(solution);

parameterized_tests! {
    day12_part1_example_a: solution(r#"[1,2,3]"#) == 6,
    day12_part1_example_b: solution(r#"{"a":2,"b":4}"#) == 6,
    day12_part1_example_c: solution(r#"[[[3]]]"#) == 3,
    day12_part1_example_d: solution(r#"{"a":{"b":4},"c":-1}"#) == 3,
    day12_part1_example_e: solution(r#"{"a":[-1,1]}"#) == 0,
    day12_part1_example_f: solution(r#"[-1,{"a":1}]"#) == 0,
    day12_part1_example_g: solution(r#"[]"#) == 0,
    day12_part1_example_h: solution(r#"{}"#) == 0,
}