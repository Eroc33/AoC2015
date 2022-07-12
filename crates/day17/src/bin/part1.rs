use std::collections::HashSet;
use std::io::BufRead;

use shared::anyhow;
use shared::combine::{sep_end_by1, EasyParser, Parser, Stream};
use shared::parse::usize;

pub fn parser<Input>() -> impl Parser<Input, Output = Vec<usize>>
where
    Input: Stream<Token = char>,
{
    sep_end_by1(usize(), shared::parse::lax_newline())
}

fn solution(mut input: impl BufRead) -> shared::Result<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (container_sizes, rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);
    
    Ok(fill_containers(&container_sizes, 150).len())
}

fn fill_containers(containers: &[usize], total: usize) -> HashSet<Vec<bool>> {
    fn fill_containers_inner(
        containers: &[usize],
        available: Vec<bool>,
        total: usize,
    ) -> HashSet<Vec<bool>> {
        if available.is_empty() {
            return Default::default();
        }

        if total == 0 {
            return std::iter::once(available.iter().map(|on| !on).collect()).collect();
        }

        let mut sum = HashSet::new();
        for i in available.iter().enumerate().filter_map(|(i,&on)| if on { Some(i) } else { None }) {
            let selected = containers[i];
            if selected <= total {
                let mut remaining = available.clone();
                remaining[i] = false;
                sum.extend(fill_containers_inner(
                    containers,
                    remaining,
                    total - selected,
                ));
            }
        }
        return sum;
    }
    return fill_containers_inner(
        containers,
        containers.iter().map(|_| true).collect(),
        total,
    );
}

fn unmap_bits<T: Copy>(original: &[T], bits: &[bool]) -> Vec<T>{
    assert_eq!(original.len(), bits.len());
    bits.iter().enumerate().filter_map(|(i,&on)|{
        if on {
            Some(original[i])
        }else{
            None
        }
    }).collect()
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day17_part1_example() {
    let containers = vec![20, 15, 10, 5, 5];
    assert_eq!(
        fill_containers(&containers, 25).into_iter().map(|bits| unmap_bits(&containers, &bits)).collect::<Vec<_>>(),
        vec![vec![20, 5], vec![15, 5, 5], vec![20, 5], vec![15, 10]]
    );
}
