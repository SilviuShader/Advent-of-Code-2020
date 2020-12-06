// AGAIN, ADD A \n TO THE END OF THE INPUT FILE

use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines : Vec<&str> = contents.split("\n").collect();

    let mut crt_set = HashSet::new();
    let mut crt_map = HashMap::new();
    let mut result1 = 0;
    let mut result2 = 0;

    let mut lines_count = 0;

    for line in lines {
        if line.len() > 1 {
            for chr in line.chars() {
                if chr >= 'a' && chr <= 'z' {
                    crt_set.insert(chr);
                    if crt_map.get(&chr) == None {
                        crt_map.insert(chr, 1);
                    } else {
                        *crt_map.get_mut(&chr).unwrap() += 1;
                    }
                }
            }

            lines_count = lines_count + 1;
        } else {

            for (_, v) in crt_map {
                if v == lines_count {
                    result2 = result2 + 1;
                }
            }

            result1 = result1 + crt_set.len();
            crt_set = HashSet::new();
            crt_map = HashMap::new();
            lines_count = 0;
        }
    }

    println!("{} {}", result1, result2);
}
