use std::io::{self, Read};

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut total = 0;

    for group in buffer.split("\n\n") {
        let mut set: HashSet<char> = HashSet::new();
        let mut first = true;

        for line in group.split_ascii_whitespace() {
            let mut inset: HashSet<char> = HashSet::new();

            if line.len() == 0 {
                continue;
            }

            for char in line.chars() {
                match char {
                    'a'..='z' => {
                        inset.insert(char);
                    }
                    _ => continue,
                }
            }

            if first {
                set = inset;
                first = false;
            } else {
                set = HashSet::from_iter(set.intersection(&inset).into_iter().map(|c| c.clone()));
            }
        }

        total += set.len();
    }

    println!("{}", total);
    Ok(())
}
