use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut good_count = 0;

    for line in buffer.lines() {
        let parts: Vec<&str> = line.split(&['-',' ',':'][..]).collect();

        let position1: usize = parts[0].parse::<usize>().expect("Unable to parse low") - 1;
        let position2: usize = parts[1].parse::<usize>().expect("Unable to parse high") - 1;
        let char: char = parts[2].parse().expect("Unable to parse char");
        // Due to ": ", there will be an empty string in parts[3], making
        // password parts[4]
        let password: Vec<char> = parts[4].chars().collect();

        if (password[position1] == char) ^ (password[position2] == char) {
            good_count += 1;
        }
    }

    println!("Answer: {}", good_count);

    Ok(())
}
