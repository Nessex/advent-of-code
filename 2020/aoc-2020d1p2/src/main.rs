use std::io::{self, Read};
use itertools::Itertools;

const TARGET: u32 = 2020;

fn parse_nums(input: String) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();

    for line in input.split_ascii_whitespace() {
        let num: u32 = match line.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        nums.push(num);
    }

    nums
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let nums = parse_nums(buffer);

    // Instead of the naive implementation of part 1,
    // I'm using itertools' Combinations to ensure fewer iterations.
    // https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.combinations
    // source: https://docs.rs/itertools/0.9.0/src/itertools/combinations.rs.html#48
    //
    // Itertools implements a scheme similar to those described here:
    // https://stackoverflow.com/a/127856
    let it = nums.iter().combinations(3);

    for i in it {
        if i[0] + i[1] + i[2] == TARGET {
            println!("Answer: {}", i[0] * i[1] * i[2]);
            break;
        }
    }

    Ok(())
}
