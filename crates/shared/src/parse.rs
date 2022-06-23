


use combine::{
    parser::{char::{digit, newline, string}, combinator::no_partial}, Parser, Stream, ParseError, many1, StreamOnce,
    error::StreamError, optional, satisfy,
};

pub fn u32<Input>() -> impl Parser<Input, Output = u32>
    where Input: Stream<Token = char>,

{
    many1(digit()).and_then(|bs: String| {
        bs.parse::<u32>().map_err(|e| <<Input as StreamOnce>::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError::other(e) )
    })
}

pub fn usize<Input>() -> impl Parser<Input, Output = usize> 
    where Input: Stream<Token = char>,
{
    many1(digit()).and_then(|bs: String| {
        bs.parse::<usize>().map_err(|e| <<Input as StreamOnce>::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError::other(e) )
    })
}

pub fn u16<Input>() -> impl Parser<Input, Output = u16> 
    where Input: Stream<Token = char>,
{
    many1(digit()).and_then(|bs: String| {
        bs.parse::<u16>().map_err(|e| <<Input as StreamOnce>::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError::other(e) )
    })
}

pub fn i64<Input>() -> impl Parser<Input, Output = i64> 
    where Input: Stream<Token = char>,
{
    (optional(string("-")),many1(digit())).and_then(|(minus, bs): (Option<&str>, String)| {
        let str = minus.unwrap_or_else(|| "").to_owned() + &bs;
        str.parse::<i64>().map_err(|e| <<Input as StreamOnce>::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError::other(e) )
    })
}

pub fn lax_newline<Input>() -> impl Parser<Input, Output = char> 
    where Input: Stream<Token = char>,
{
    no_partial(optional(satisfy(|ch: char| ch == '\r')).with(newline())).expected("crlf or lf newline")
}