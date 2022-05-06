use std::io::BufRead;

use shared::parameterized_tests;

fn has_three_vowels(line: &str) -> bool{
    line.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn has_two_in_a_row(line: &str) -> bool{
    for (a,b) in line.chars().zip(line.chars().skip(1)){
        if a == b {
            return true;
        }
    }
    return false;
}

fn has_naughty_pairs(line: &str) -> bool {
    const NAUGHTY_PAIRS: &[(char,char)] = &[
        ('a','b'),
        ('c','d'),
        ('p','q'),
        ('x','y'),
    ];
    for (a,b) in line.chars().zip(line.chars().skip(1)){
        if NAUGHTY_PAIRS.contains(&(a,b)) {
            return true;
        }
    }
    return false;
}

fn solution(input: impl BufRead) -> shared::Result<i32> {
    let mut count = 0;
    for line in input.lines(){
        let line = line?;
        let is_nice = has_three_vowels(&line) 
            && has_two_in_a_row(&line)
            && !has_naughty_pairs(&line);
        if is_nice{
            count += 1;
        }
    }

    Ok(count)
}

shared::main!(solution);

parameterized_tests! {
    day05_part1_example_a: solution("ugknbfddgicrmopn") == 1,
    day05_part1_example_b: solution("aaa") == 1,
    day05_part1_example_c: solution("jchzalrnumimnmhp") == 0,
    day05_part1_example_d: solution("haegwjzuvuyypxyu") == 0,
    day05_part1_example_e: solution("dvszwmarrgswjxmb") == 0,
}
