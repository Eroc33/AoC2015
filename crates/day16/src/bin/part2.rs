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
pub struct Analysis {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl From<HashMap<String, usize>> for Analysis {
    fn from(map: HashMap<String, usize>) -> Self {
        Self {
            children: map.get("children").copied(),
            cats: map.get("cats").copied(),
            samoyeds: map.get("samoyeds").copied(),
            pomeranians: map.get("pomeranians").copied(),
            akitas: map.get("akitas").copied(),
            vizslas: map.get("vizslas").copied(),
            goldfish: map.get("goldfish").copied(),
            trees: map.get("trees").copied(),
            cars: map.get("cars").copied(),
            perfumes: map.get("perfumes").copied(),
        }
    }
}

pub fn parser<Input>() -> impl Parser<Input, Output = HashMap<usize, Analysis>>
where
    Input: Stream<Token = char>,
{
    sep_end_by1(
        (
            string("Sue ").with(usize()).skip(string(": ")),
            sep_by1((many1(letter()).skip(string(": ")), usize()), string(", "))
                .map(<Analysis as From<HashMap<String, usize>>>::from),
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

    let target_info = Analysis {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let candidates: Vec<_> = aunts
        .into_iter()
        .filter(|(_n, aunt)| {
            let mut mismatch = false;
            let cmps: &[(
                &Option<usize>,
                &Option<usize>,
                &dyn Fn(&usize, &usize) -> bool,
            )] = &[
                (&aunt.children, &target_info.children, &|a, b| a == b),
                (&aunt.cats, &target_info.cats, &|a, b| a > b),
                (&aunt.samoyeds, &target_info.samoyeds, &|a, b| a == b),
                (&aunt.pomeranians, &target_info.pomeranians, &|a, b| a < b),
                (&aunt.akitas, &target_info.akitas, &|a, b| a == b),
                (&aunt.vizslas, &target_info.vizslas, &|a, b| a == b),
                (&aunt.goldfish, &target_info.goldfish, &|a, b| a < b),
                (&aunt.trees, &target_info.trees, &|a, b| a > b),
                (&aunt.cars, &target_info.cars, &|a, b| a == b),
                (&aunt.perfumes, &target_info.perfumes, &|a, b| a == b),
            ];
            for (a, b, cmp) in cmps {
                if let (Some(a), Some(b)) = (a, b) {
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
