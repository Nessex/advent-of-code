use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut good_count = 0;

    for line in buffer.lines() {
        let parts: Vec<&str> = line.split(&['-',' ',':'][..]).collect();

        let low: usize = parts[0].parse().expect("Unable to parse low");
        let high: usize = parts[1].parse().expect("Unable to parse high");
        let char: char = parts[2].parse().expect("Unable to parse char");
        // Due to ": ", there will be an empty string in parts[3], making
        // password parts[4]
        let password = parts[4];

        let count = password.chars().filter(|c| *c == char).count();

        if count >= low && count <= high {
            good_count += 1;
        }

    }

    println!("Answer: {}", good_count);

    Ok(())
}
