use shared::combine::{choice, many1, token, Parser};

pub type Direction = [i64; 2];

fn left() -> Direction {
    [-1, 0]
}

fn right() -> Direction {
    [1, 0]
}

fn up() -> Direction {
    [0, 1]
}

fn down() -> Direction {
    [0, -1]
}

pub fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Direction>> {
    many1(choice((
        token('<').map(|_| left()),
        token('>').map(|_| right()),
        token('^').map(|_| up()),
        token('v').map(|_| down()),
    )))
}
