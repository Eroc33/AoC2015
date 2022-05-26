use std::io::BufRead;

fn runs<T: Copy + PartialEq>(mut i: impl Iterator<Item = T>) -> impl Iterator<Item = (T,usize)>{
    let mut current = None;
    let mut current_count = 1;
    std::iter::from_fn(move ||{
        while let Some(item) = i.next(){
            if Some(item) == current {
                current_count += 1;
            }else {
                let ret = current.take().map(|curr| (curr, current_count));
                current = Some(item);
                current_count = 1;
                if let Some(ret) = ret{
                    return Some(ret);
                }
            }
        }
        if let Some(curr) = current.take(){
            return Some((curr, current_count));
        }
        None
    })
}

fn look_and_say(buf: &str) -> String {
    runs(buf.chars()).flat_map(|(item,count)|{
        [count.to_string(), item.to_string()]
    }).collect()
}

fn solution(mut input: impl BufRead) -> shared::Result<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    for _ in 0..40{
        buf = look_and_say(&buf);
    }

    Ok(buf.len())
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day10_part1_example_a() {
    assert_eq!(look_and_say("1"), "11")
}
#[cfg(test)]
#[test]
fn day10_part1_example_b() {
    assert_eq!(look_and_say("11"), "21")
}
#[cfg(test)]
#[test]
fn day10_part1_example_c() {
    assert_eq!(look_and_say("21"), "1211")
}
#[cfg(test)]
#[test]
fn day10_part1_example_d() {
    assert_eq!(look_and_say("1211"), "111221")
}
#[cfg(test)]
#[test]
fn day10_part1_example_e() {
    assert_eq!(look_and_say("111221"), "312211")
}