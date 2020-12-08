use std::io::{self, Read};

use itertools::Itertools;
use regex::Regex;
use anyhow::Result;

#[derive(Debug)]
struct Invalid;
impl Into<anyhow::Error> for Invalid {
    fn into(self) -> anyhow::Error {
        anyhow::Error::msg("invalid")
    }
}
impl Invalid {
    fn err() -> anyhow::Error {
        anyhow::Error::msg("invalid")
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;



    println!("{}", total);
    Ok(())
}
