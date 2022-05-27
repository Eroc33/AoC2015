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
                if object.values().any(|val| val == "red"){
                    continue;
                }
                open.extend(object.into_iter().map(|(_k,v)| v));
            }
        }
    }
    Ok(nums.into_iter().sum())
}

shared::main!(solution);

parameterized_tests! {
    day12_part2_example_a: solution(r#"[1,2,3]"#) == 6,
    day12_part2_example_b: solution(r#"[1,{"c":"red","b":2},3]"#) == 4,
    day12_part2_example_c: solution(r#"{"d":"red","e":[1,2,3,4],"f":5}"#) == 0,
    day12_part2_example_d: solution(r#"[1,"red",5]"#) == 6,
}