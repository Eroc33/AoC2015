use combine::{
    error::StringStreamError,
    parser::{char::digit, range::recognize},
    skip_many1, Parser,
};

pub fn u32<'a>() -> impl Parser<&'a str, Output = u32> {
    recognize(skip_many1(digit())).and_then(|bs: &'a str| {
        bs.parse::<u32>()
            .map_err(|_| StringStreamError::UnexpectedParse)
    })
}
