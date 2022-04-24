use std::{
    env,
    fmt::Debug,
    fs,
    io::{BufRead, BufReader},
};

pub use anyhow::{anyhow, bail, ensure, Error, Result};

#[macro_export]
macro_rules! main {
    ($solution_fn:ident) => {
        fn main() -> $crate::Result<()> {
            let solution = $solution_fn($crate::input()?)?;
            println!("{}", solution);
            Ok(())
        }
    };
}

pub fn input() -> Result<impl BufRead> {
    let filename = match env::args().nth(1) {
        Some(v) => v,
        None => {
            bail!("You must pass a filename as first argument");
        }
    };
    match fs::File::open(filename) {
        Ok(v) => Ok(BufReader::new(v)),
        Err(e) => {
            bail!("Couldn't open input file due to io error: {:?}", e);
        }
    }
}

pub fn check_example<'a, F, T>(solution: F, input: &'a str, value: T)
where
    F: FnOnce(std::io::Cursor<&'a str>) -> Result<T, Error>,
    T: PartialEq + Debug,
{
    assert_eq!(solution(std::io::Cursor::new(input)).unwrap(), value)
}
