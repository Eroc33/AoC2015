use std::fmt::Debug;
use std::io::BufRead;

use shared::anyhow;
use shared::combine::{sep_end_by1, EasyParser, Parser, Stream, choice, token, many1};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    On,
    Off,
}

pub fn parser<Input>() -> impl Parser<Input, Output = Vec<Vec<Cell>>>
where
    Input: Stream<Token = char>,
{

    let cell_parser = choice((
        token('#').map(|_| Cell::On),
        token('.').map(|_| Cell::Off),
    ));
    sep_end_by1(many1(cell_parser), shared::parse::lax_newline())
}
#[derive(Clone)]
pub struct Map{
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Debug for Map{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.height{
            let line = &self.cells[self.width*y..][..self.width];
            writeln!(f,"{}",line.iter().map(|cell| if *cell == Cell::On {'#'}else{'.'}).collect::<String>())?;
        }
        Ok(())
    }
}

impl Map{
    pub fn new(cells: Vec<Cell>, width: usize, height: usize) -> Self {
        assert_eq!(cells.len(), width*height);
        Self { width, height, cells }
    }

    fn iter_coordinates(&self) -> impl Iterator<Item=(usize,usize)>{
        let width = self.width;
        let height = self.height;
        (0..height).into_iter().flat_map(move |y|{
            (0..width).into_iter().map(move |x|{
                (x,y)
            })
        })
    }

    fn get(&self, coords: (usize,usize)) -> Option<&Cell>{
        let index = self.width*coords.1+coords.0;
        self.cells.get(index)
    }

    fn set(&mut self, coords: (usize,usize), cell: Cell){
        let index = self.width*coords.1+coords.0;
        self.cells[index] = cell;
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize{
        let min_x = if x==0 {0} else {x-1};
        let min_y = if y==0 {0} else {y-1};
        let max_x = if x==self.width-1 {x} else {x+1};
        let max_y = if y==self.height-1 {y} else {y+1};
        let orig_x = x;
        let orig_y = y;

        let mut count = 0;
        for y in min_y..=max_y{
            for x in min_x..=max_x{
                if y==orig_y && x==orig_x{
                    continue;
                }
                if let Some(Cell::On) = self.get((x,y)){
                    count += 1;
                }
            }
        }
        count
    }
}

fn turn_corners_on(map: &mut Map){
    map.set((0,0), Cell::On);
    map.set((map.width-1,0), Cell::On);
    map.set((0,map.height-1), Cell::On);
    map.set((map.width-1,map.height-1), Cell::On);
}

fn simulate_and_count(mut input: impl BufRead, cycles: usize) -> shared::Result<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (cells, rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    let height = cells.len();
    let width = cells.first().map_or(0, |line| line.len());
    assert!(cells.iter().all(|line| line.len() == width), "lines are different widths!");

    let cells: Vec<Cell> = cells.into_iter().flatten().collect();
    let mut map = Map::new(cells, width, height);
    turn_corners_on(&mut map);
    let mut new_map = map.clone();

    for _ in 0..cycles{
        for (x,y) in map.iter_coordinates(){
            let state = match (map.get((x,y)), map.count_neighbours(x, y)){
                (Some(Cell::On), count) if count == 2 || count == 3 => {
                    Cell::On
                }
                (Some(Cell::Off), count) if count == 3 => {
                    Cell::On
                }
                (Some(_),_) =>{
                    Cell::Off
                }
                (None, _) => {
                    panic!("Out of bounds");
                }
            };
            new_map.set((x,y), state);
        }
        turn_corners_on(&mut new_map);
        std::mem::swap(&mut map, &mut new_map);
    }

    Ok(map.cells.iter().filter(|cell| **cell == Cell::On).count())
}

fn solution(input: impl BufRead) -> shared::Result<usize> {
    simulate_and_count(input, 100)
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day18_part2_example() {
    shared::check_example(|input| simulate_and_count(input, 5), 
".#.#.#
...##.
#....#
..#...
#.#..#
####..", 17)
}
