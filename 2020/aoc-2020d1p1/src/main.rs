use std::io::{self, Read};

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

    'outer: for (i1, n1) in nums.iter().enumerate() {
        for (i2, n2) in nums.iter().enumerate() {
            if i1 == i2 {
                continue;
            }

            if n1 + n2 == TARGET {
                println!("Answer: {}", n1 * n2);
                break 'outer;
            }
        }
    }

    Ok(())
}
