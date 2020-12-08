use std::io::{self, Read};

use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut total = 0;

    for line in buffer.split("\n\n") {
        let mut set: HashSet<char> = HashSet::new();

        for char in line.chars() {
            match char {
                'a'..='z' => {
                    set.insert(char);
                }
                _ => continue,
            }
        }

        total += set.len();
    }


    println!("{}", total);
    Ok(())
}
