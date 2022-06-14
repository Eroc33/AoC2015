use std::{io::BufRead, collections::{BTreeSet, HashMap}};

use petgraph::{Undirected, graph::NodeIndex};
use shared::{combine::{*, parser::char::string}, anyhow};

pub fn parser<Input>() -> impl Parser<Input, Output = Vec<(String,String,isize)>>
where
    Input: Stream<Token = char>,
{
    sep_by1((
        many1(satisfy(|c| c != ' ')),
        string(" would "),
        choice((
            string("gain").map(|_| 1isize),
            string("lose").map(|_| -1isize),
        )),
        string(" "),
        shared::parse::usize(),
        string(" happiness units by sitting next to "),
        many1(satisfy(|c| c != ' ' && c != '.')),
        string("."),
    ).map(|(from,_,mult,_,dist,_, to,_)| (from,to,mult*(dist as isize))),shared::parse::lax_newline())
}

fn solution(mut input: impl BufRead) -> shared::Result<isize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (edges, rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    let nodes: BTreeSet<_> = edges.iter().flat_map(|(a,b,_)| [a.clone(),b.clone()]).collect();
    let node_weights: HashMap<_,_> = edges.iter().map(|(a,b,w)| ((a.clone(),b.clone()),*w)).collect();

    let edges_with_indexes = edges.into_iter().map(|(a,b,w)| 
        (
            nodes.iter().position(|n| *n == a).expect("node not in position set") as u32,
            nodes.iter().position(|n| *n == b).expect("node not in position set") as u32, 
            w+*node_weights.get(&(b,a)).unwrap()
        )
    );

    let graph = petgraph::Graph::<String, isize, Undirected>::from_edges(edges_with_indexes);

    let mut all_paths: Vec<Vec<NodeIndex>> = vec![];
    for (i,_start) in nodes.iter().enumerate(){
        for (j,_end) in nodes.iter().enumerate(){
            for mut path in petgraph::algo::simple_paths::all_simple_paths::<Vec<NodeIndex>,_>(&graph,NodeIndex::from(i as u32), NodeIndex::from(j as u32), nodes.len() - 2, Some(nodes.len() - 2)){
                // complete cycle for accurate weights
                path.push(NodeIndex::new(i));
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
            if best_score.map_or(true, |bs| score > bs){
                best_score = Some(score);
                best = Some(path);
            }
        }
    }

    dbg!(best.map(|best| best.iter().map(|idx| nodes.iter().nth(idx.index()).unwrap()).collect::<Vec<_>>()));

    Ok(best_score.expect("No best"))
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day13_part1_example() {
    shared::check_example(solution, 
r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#, 330)
}
