use std::{io::BufRead};

use shared::parameterized_tests;

fn has_nonoverlapping_repeated_pair(line: &str) -> bool{
    for i in 0..line.chars().count(){
        let pair = (line.chars().nth(i),line.chars().nth(i+1));
        let pair = if let (Some(a),Some(b)) = pair{
            (a,b)
        } else {
            break
        };
        let mut following_pairs = line.chars().skip(i+2).zip(line.chars().skip(i+3));
        if following_pairs.any(|x| x == pair){
            return true;
        }
    }
    false
}

fn has_skipped_repeat(line: &str) -> bool{
    for (a,c) in line.chars().zip(line.chars().skip(2)) {
        if a == c {
            return true;
        }
    }
    return false;
}

fn solution(input: impl BufRead) -> shared::Result<i32> {
    let mut count = 0;
    for line in input.lines(){
        let line = line?;
        let is_nice = has_nonoverlapping_repeated_pair(&line) 
            && has_skipped_repeat(&line);
        if is_nice{
            count += 1;
        }
    }

    Ok(count)
}

shared::main!(solution);

parameterized_tests! {
    day05_part2_example_a: solution("qjhvhtzxzqqjkmpb") == 1,
    day05_part2_example_b: solution("xxyxx") == 1,
    day05_part2_example_c: solution("uurcxstgmygtbstg") == 0,
    day05_part2_example_d: solution("ieodomkazucvgmuy") == 0,
}
