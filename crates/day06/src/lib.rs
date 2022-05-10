use shared::{combine::{choice, parser::char::string, Parser, Stream, attempt}};
use std::{ops::RangeInclusive};

#[derive(Clone)]
pub enum Action {
    On,
    Off,
    Toggle,
}

impl Action {
    pub fn parser<Input>() -> impl Parser<Input, Output = Action> 
        where Input: Stream<Token = char>
    {
        choice((
            attempt(string("turn off").map(|_| Action::Off)),
            attempt(string("turn on").map(|_| Action::On)),
            attempt(string("toggle").map(|_| Action::Toggle)),
        ))
    }
}

#[derive(Clone)]
pub struct Command{
    pub action: Action,
    pub x_range: RangeInclusive<usize>,
    pub y_range: RangeInclusive<usize>,
}

impl Command{
    pub fn parser<Input>() -> impl Parser<Input, Output = Command>
        where Input: Stream<Token = char>,
    {
        (
            Action::parser(),
            string(" "),
            shared::parse::usize(),
            string(","),
            shared::parse::usize(),
            string(" through "),
            shared::parse::usize(),
            string(","),
            shared::parse::usize(),
        )
            .map(|(action, _, x1, _, y1, _, x2, _, y2)| {
                Command{
                    action,
                    x_range: x1..=x2,
                    y_range: y1..=y2,
                }
            })
    }
}