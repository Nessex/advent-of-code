use std::io::{self, Read};
use crate::Tile::TREE;

#[derive(PartialEq,Eq)]
enum Tile {
    EMPTY,
    TREE,
}

struct Map {
    data: Vec<Vec<Tile>>,
    row_len: usize,
}

impl Map {
    fn new(raw_data: String) -> Self {
        let mut data: Vec<Vec<Tile>> = Vec::new();
        let mut row_len: usize = 0;

        for line in raw_data.lines() {
            let chars: Vec<char> = line.chars().collect();

            if chars.len() == 0 {
                break;
            }

            row_len = chars.len();

            let mut tiles: Vec<Tile> = Vec::new();

            for c in chars {
                match c {
                    '#' => tiles.push(Tile::TREE),
                    '.' => tiles.push(Tile::EMPTY),
                    _ => break,
                }
            }

            data.push(tiles);
        }

        Self {
            data,
            row_len,
        }
    }

    fn count_trees_in_slope(&self, across: usize, down: usize) -> u64 {
        let mut count: u64 = 0;
        let mut cur_x: usize = 0;

        for row in self.data.iter().step_by(down).skip(1) {
            cur_x = (cur_x + across) % self.row_len;

            if row[cur_x] == TREE {
                count += 1;
            }
        }

        count
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let map = Map::new(buffer);

    let counts: Vec<u64> = vec![
        map.count_trees_in_slope(1, 1),
        map.count_trees_in_slope(3, 1),
        map.count_trees_in_slope(5, 1),
        map.count_trees_in_slope(7, 1),
        map.count_trees_in_slope(1, 2),
    ];

    let total = counts.iter().fold(1, |acc, v| acc * v);

    println!("Answer: {}", total);

    Ok(())
}
