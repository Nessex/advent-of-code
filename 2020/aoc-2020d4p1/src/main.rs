use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut valid_count = 0;

    let passports = buffer.split("\n\n");

    for passport in passports {
        let mut byr = false;
        let mut iyr = false;
        let mut eyr = false;
        let mut hgt = false;
        let mut hcl = false;
        let mut ecl = false;
        let mut pid = false;

        for parts in passport.split_ascii_whitespace() {
            let mut subparts: Vec<&str> = parts.split(':').collect();

            if subparts.len() != 2 {
                continue
            }

            match subparts[0] {
                "byr" => byr = true,
                "iyr" => iyr = true,
                "eyr" => eyr = true,
                "hgt" => hgt = true,
                "hcl" => hcl = true,
                "ecl" => ecl = true,
                "pid" => pid = true,
                _ => continue,
            }
        }

        if byr & iyr & eyr & hgt & hcl & ecl & pid {
            valid_count += 1;
        }
    }

    println!("Answer: {}", valid_count);
    Ok(())
}
