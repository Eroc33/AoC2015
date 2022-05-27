use std::{io::BufRead, collections::HashSet};

fn has_multiple_different_pairs(line: &[u8]) -> bool{
    let mut seen_pairs = HashSet::new();
    for pair in line.windows(2){
        if pair[0] != pair[1] {
            continue;
        }
        if seen_pairs.len() >= 1 && !seen_pairs.contains(&pair[0]) {
            return true;
        }
        seen_pairs.insert(pair[0]);
    }
    false
}

fn is_acceptable_password(buf: &[u8]) -> bool {
    buf.windows(3).any(|window| window[0]+1 == window[1] && window[1]+1 == window[2])
    && !buf.iter().any(|c| b"iol".contains(c))
    && has_multiple_different_pairs(buf)
}

fn increment(buf: &mut [u8]){
    let mut i = buf.len() - 1;
    loop{
        if buf[i] == b'z' {
            buf[i] = b'a';
            if i == 0 {
                break;
            }
            i -= 1;
        }else{
            buf[i] += 1;
            break;
        }
    }
}

fn solution(mut input: impl BufRead) -> shared::Result<String> {
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;

    while !is_acceptable_password(&buf){
        increment(&mut buf)
    }

    dbg!(
        buf.windows(3).any(|window| window[0]+1 == window[1] && window[1]+1 == window[2]),
        !buf.iter().any(|c| b"iol".contains(c)),
        buf.windows(2).any(|window| window[0] == window[1]),
    );

    Ok(String::from_utf8_lossy(&buf).into_owned())
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day11_test_increment() {
    let mut var: Vec<u8> = b"xx".to_vec();
    increment(&mut var);
    assert_eq!(var,b"xy");
    increment(&mut var);
    assert_eq!(var,b"xz");
    increment(&mut var);
    assert_eq!(var,b"ya");
    increment(&mut var);
    assert_eq!(var,b"yb");
}


#[cfg(test)]
#[test]
fn day11_part1_example() {
    shared::check_example(solution, "abcdefgh", "abcdffaa".to_string())
}
