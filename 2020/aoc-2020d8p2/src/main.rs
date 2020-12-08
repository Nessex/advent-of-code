use std::io::{self, Read};

use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut lines: Vec<(&str, i32)> = Vec::new();

    for line in buffer.lines() {
        let mut l = line.split_ascii_whitespace();
        let op = l.next().unwrap();
        let num: i32 = l.next().unwrap().parse().unwrap();

        lines.push((op, num));
    }

    // Super naive approach: for each run, replace one line's jmp with nop, nop with jmp
    // and try all lines till we hit the win condition.
    'outer: for (i, _) in lines.iter().enumerate() {
        // Reset everything
        let mut accum = 0;
        let mut lrun: HashSet<i32> = HashSet::new();
        let mut pnt: i32 = 0;

        loop {
            // We win if we hit the line after the last line in the file
            if pnt == lines.len() as i32 {
                println!("Answer: {}", accum);
                break 'outer;
            }

            // Detect infinite loops
            if lrun.get(&pnt).is_some() {
                continue 'outer;
            }

            let (op, num) = lines.get(pnt as usize).unwrap();
            lrun.insert(pnt);

            // Replace the current line's op
            let r_op = if i as i32 == pnt {
                match op {
                    &"nop" => "jmp",
                    &"jmp" => "nop",
                    _ => op,
                }
            } else {
                op
            };

            match r_op {
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
    }

    Ok(())
}
