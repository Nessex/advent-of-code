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

    let mut max_id = 0;

    for line in buffer.lines() {
        let mut r_high = 127;
        let mut r_low = 0;
        let mut c_high = 7;
        let mut c_low = 0;
        let mut id = 0;

        for char in line.chars() {
            match char {
                'F' => {
                    r_high = r_low + ((r_high - r_low) / 2);
                },
                'B' => r_low = r_high - ((r_high - r_low) / 2),
                'R' => c_low = c_high - ((c_high - c_low) / 2),
                'L' => c_high = c_low + ((c_high - c_low) / 2),
                _ => continue,
            }
        }

        id = r_high * 8 + c_high;

        if id > max_id {
            max_id = id;
        }

        println!("{} {} {}", r_high, c_high, id);
    }

    println!("Answer: {}", max_id);
    Ok(())
}
