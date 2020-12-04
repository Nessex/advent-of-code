use std::io::{self, Read};
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

fn validate_byr(value: &str) -> Result<()> {
    let yr: u32 = value.parse()?;

    if yr < 1920 || yr > 2002 {
        Err(Invalid.into())
    } else {
        Ok(())
    }
}

fn validate_iyr(value: &str) -> Result<()> {
    let yr: u32 = value.parse()?;

    if yr < 2010 || yr > 2020 {
        Err(Invalid.into())
    } else {
        Ok(())
    }
}

fn validate_eyr(value: &str) -> Result<()> {
    let yr: u32 = value.parse()?;

    if yr < 2020 || yr > 2030 {
        Err(Invalid.into())
    } else {
        Ok(())
    }
}

fn validate_hgt(value: &str) -> Result<()> {
    let hgt_regex = Regex::new(r"^(?P<h>\d+)(?P<u>in|cm)$").unwrap();
    let mut it = hgt_regex.captures_iter(value);
    let capture = it.next().ok_or(Invalid::err())?;
    let height: u32 = capture["h"].parse()?;
    let units: &str = &capture["u"];
    if let Some(_overflow) = it.next() {
        return Err(Invalid.into());
    };

    match units {
        "in" => {
            if height < 59 || height > 76 {
                return Err(Invalid.into());
            }
        }
        "cm" => {
            if height > 193 || height < 150 {
                return Err(Invalid.into());
            }
        }
        _ => return Err(Invalid.into()),
    }

    Ok(())
}

fn validate_hcl(value: &str) -> Result<()> {
    let mut it = value.chars();
    let first = it.next().ok_or(Invalid::err())?;

    if first != '#' {
        return Err(Invalid.into());
    }

    let mut valid_count = 0;

    for c in it {
        if c.is_ascii_hexdigit() {
            valid_count += 1;
        }
    }

    if valid_count == 6 {
        Ok(())
    } else {
        Err(Invalid.into())
    }
}

fn validate_ecl(value: &str) -> Result<()> {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
        _ => Err(Invalid.into())
    }
}

fn validate_pid(value: &str) -> Result<()> {
    let chars: Vec<char> = value.chars().collect();

    if chars.len() != 9 {
        return Err(Invalid.into());
    }

    for c in chars {
        if !c.is_ascii_digit() {
            return Err(Invalid.into())
        }
    }

    Ok(())
}

fn validate_passport(passport: &str) -> Result<()> {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;

    for parts in passport.split_ascii_whitespace() {
        let subparts: Vec<&str> = parts.split(':').collect();

        if subparts.len() != 2 {
            continue
        }

        match subparts[0] {
            "byr" => byr = validate_byr(subparts[1]).is_ok(),
            "iyr" => iyr = validate_iyr(subparts[1]).is_ok(),
            "eyr" => eyr = validate_eyr(subparts[1]).is_ok(),
            "hgt" => hgt = validate_hgt(subparts[1]).is_ok(),
            "hcl" => hcl = validate_hcl(subparts[1]).is_ok(),
            "ecl" => ecl = validate_ecl(subparts[1]).is_ok(),
            "pid" => pid = validate_pid(subparts[1]).is_ok(),
            _ => continue,
        }
    }

    if byr & iyr & eyr & hgt & hcl & ecl & pid {
        Ok(())
    } else {
        Err(Invalid.into())
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut valid_count = 0;

    let passports = buffer.split("\n\n");

    for passport in passports {
        match validate_passport(passport) {
            Ok(_) => valid_count += 1,
            Err(_) => continue,
        }
    }

    println!("Answer: {}", valid_count);
    Ok(())
}
