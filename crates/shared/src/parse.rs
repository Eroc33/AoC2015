use combine::{
    parser::{char::digit}, Parser, Stream, ParseError, many1, StreamOnce,
    error::StreamError,
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
