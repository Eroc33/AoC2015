pub mod parse;

use std::{
    env,
    fmt::Debug,
    fs,
    io::{BufRead, BufReader},
};

pub use anyhow::{anyhow, bail, ensure, Error, Result};
pub use combine;

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
            if let Ok(pkg) = env::var("CARGO_PKG_NAME"){
                format!("./inputs/{}.txt",pkg)
            }else{
                bail!("You must pass a filename as first argument");
            }
        }
    };
    match fs::File::open(&filename) {
        Ok(v) => Ok(BufReader::new(v)),
        Err(e) => {
            bail!("Couldn't open input file ({}) due to io error: {:?}", filename, e);
        }
    }
}

pub fn check_example<'a, F, T>(solution: F, input: &'a str, value: T)
where
    F: FnOnce(std::io::Cursor<&'a str>) -> Result<T, Error>,
    T: PartialEq + Debug,
{
    let val = match solution(std::io::Cursor::new(input)){
        Ok(val) => val,
        Err(e) => panic!("Err: {}", e),
    };
    assert_eq!(val, value)
}

#[macro_export]
macro_rules! parameterized_tests {
    ($($name:ident: $solution:ident($input:expr) == $value:expr,)*) => {
    $(
        #[cfg(test)]
        #[test]
        fn $name() {
            shared::check_example($solution, $input, $value)
        }
    )*
    }
}
