use std::{collections::HashMap, io::BufRead};

use shared::{
    anyhow,
    combine::{
        parser::char::{letter, string},
        *,
    },
    parse::usize,
};

#[derive(Debug)]
pub struct Analysis(HashMap<String, usize>);

pub fn parser<Input>() -> impl Parser<Input, Output = HashMap<usize, Analysis>>
where
    Input: Stream<Token = char>,
{
    sep_end_by1(
        (
            string("Sue ").with(usize()).skip(string(": ")),
            sep_by1((many1(letter()).skip(string(": ")), usize()), string(", ")).map(Analysis),
        ),
        shared::parse::lax_newline(),
    )
}

fn solution(mut input: impl BufRead) -> shared::Result<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (aunts, rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    let target_info = Analysis({
        let mut map = HashMap::new();
        map.insert("children".to_owned(), 3);
        map.insert("cats".to_owned(), 7);
        map.insert("samoyeds".to_owned(), 2);
        map.insert("pomeranians".to_owned(), 3);
        map.insert("akitas".to_owned(), 0);
        map.insert("vizslas".to_owned(), 0);
        map.insert("goldfish".to_owned(), 5);
        map.insert("trees".to_owned(), 3);
        map.insert("cars".to_owned(), 2);
        map.insert("perfumes".to_owned(), 1);
        map
    });

    let candidates: Vec<_> = aunts
        .into_iter()
        .filter(|(_n, aunt)| {
            let mut mismatch = false;
            let cmps: &[(&str, &dyn Fn(&usize, &usize) -> bool)] = &[
                ("children", &|a, b| a == b),
                ("cats", &|a, b| a > b),
                ("samoyeds", &|a, b| a == b),
                ("pomeranians", &|a, b| a < b),
                ("akitas", &|a, b| a == b),
                ("vizslas", &|a, b| a == b),
                ("goldfish", &|a, b| a < b),
                ("trees", &|a, b| a > b),
                ("cars", &|a, b| a == b),
                ("perfumes", &|a, b| a == b),
            ];
            for (key, cmp) in cmps {
                if let (Some(a), Some(b)) = (aunt.0.get(*key), target_info.0.get(*key)) {
                    if !cmp(a, b) {
                        mismatch = true
                    }
                }
            }
            !mismatch
        })
        .collect();

    assert_eq!(
        candidates.len(),
        1,
        "Should only be 1 candidate at this point"
    );

    Ok(candidates[0].0)
}

shared::main!(solution);
