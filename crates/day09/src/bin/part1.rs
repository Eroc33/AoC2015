use std::{io::BufRead, collections::{BTreeSet}};

use petgraph::{Undirected, graph::NodeIndex};
use shared::{combine::{*, parser::char::string}, anyhow};

pub fn parser<Input>() -> impl Parser<Input, Output = Vec<(String,String,usize)>>
where
    Input: Stream<Token = char>,
{
    sep_by1((
        many1(satisfy(|c| c != ' ')),
        string(" to "),
        many1(satisfy(|c| c != ' ')),
        string(" = "),
        shared::parse::usize()
    ).map(|(from,_,to,_,dist)| (from,to,dist)),shared::parse::lax_newline())
}

fn solution(mut input: impl BufRead) -> shared::Result<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (edges, rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    let nodes: BTreeSet<_> = edges.iter().flat_map(|(a,b,_)| [a.clone(),b.clone()]).collect();

    let edges_with_indexes = edges.into_iter().map(|(a,b,w)| 
        (
            nodes.iter().position(|n| *n == a).expect("node not in position set") as u32,
            nodes.iter().position(|n| *n == b).expect("node not in position set") as u32, 
            w,
        )
    );

    let graph = petgraph::Graph::<String, usize, Undirected>::from_edges(edges_with_indexes);

    let mut all_paths: Vec<Vec<_>> = vec![];
    for (i,_start) in nodes.iter().enumerate(){
        for (j,_end) in nodes.iter().enumerate(){
            for path in petgraph::algo::simple_paths::all_simple_paths(&graph,NodeIndex::from(i as u32), NodeIndex::from(j as u32), nodes.len() - 2, Some(nodes.len() - 2)){
                all_paths.push(path);
            }
        }
    }

    let score_path = |path: &[NodeIndex<u32>]|{
        let mut score = 0;
        for window in path.windows(2){
            score += graph.edge_weight(graph.find_edge(window[0], window[1]).expect("Missing weight")).expect("Missing weight");
        }
        score
    };

    let mut best = None;
    let mut best_score = None;
    for path in all_paths{
        if nodes.iter().enumerate().all(|(i,_node)| path.contains(&NodeIndex::from(i as u32))){
            let score = score_path(&path);
            if best_score.map_or(true, |bs| score < bs){
                best_score = Some(score);
                best = Some(path);
            }
        }
    }

    dbg!(best);

    Ok(best_score.expect("No best"))
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day09_part1_example() {
    shared::check_example(solution, 
r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#, 605)
}
