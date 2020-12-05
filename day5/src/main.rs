use std::fs;
use std::cmp::min;
use std::cmp::max;

struct Pos(usize, usize);

fn solve(seq : &str) -> Pos {
    let mut down = 0;
    let mut up = 127;
    let mut left = 0;
    let mut right = 7;

    for chr in seq.chars() {
        match chr {
            'B' => down = (up + down) / 2 + 1,
            'F' => up = (up + down) / 2,
            'L' => right = (left + right) / 2,
            'R' => left = (left + right) / 2 + 1,
            _ => {}
        }
    }

    Pos(min(up, down), min(left, right))
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let lines : Vec<&str> = contents.split("\n").collect();

    let mut result1 = 0;
    let mut occupied = [[false; 8] ; 128];

    for line in lines {
        let pos = solve(&line);
        occupied[pos.0][pos.1] = true;
        let val = pos.0 * 8 + pos.1;
        result1 = max(result1, val);
    }

    println!("{}", result1);

    let mut started = false;
    let mut finished = false;

    for i in 0..128 {
        if !finished {
            for j in 0..8 {
                if occupied[i][j] {
                    started = true;
                }

                if started && !occupied[i][j] {
                    println!("{} {} => {}", i, j, (i * 8) + j);
                    finished = true;
                    break;
                }
            }
        }
    }
}
