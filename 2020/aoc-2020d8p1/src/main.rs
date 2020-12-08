use std::io::{self, Read};

use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut accum = 0;
    let mut lrun: HashSet<i32> = HashSet::new();
    let mut lines: Vec<(&str, i32)> = Vec::new();
    let mut pnt: i32 = 0;

    for line in buffer.lines() {
        let mut l = line.split_ascii_whitespace();
        let op = l.next().unwrap();
        let num: i32 = l.next().unwrap().parse().unwrap();

        lines.push((op, num));
    }

    loop {
        if lrun.get(&pnt).is_some() {
            println!("Ans: {}", accum);
            break;
        }
        let (op, num) = lines.get(pnt as usize).unwrap();
        lrun.insert(pnt);

        match *op {
            "acc" => {
                accum += num;
                pnt += 1;
            },
            "jmp" => {
                pnt += num;

                if pnt < 0 {
                    pnt = pnt + lines.len() as i32;
                }
            },
            "nop" => pnt += 1,
            _ => {}
        }
    }

    Ok(())
}
