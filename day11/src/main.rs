use std::fs;

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn valid_pos(pos : (i32, i32), lines : &Vec<String>) -> bool {
    if (pos.0 >= 0 && pos.0 < lines.len() as i32) &&
       (pos.1 >= 0 && pos.1 < lines[0].len() as i32) {
           return true;
       }

    false
}

fn adjacent_count(pos : (i32, i32), lines : &Vec<String>) -> i32 {
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1), (-1, -1), (1, 1), (1, -1), (-1, 1)];
    let mut result = 0;

    for dir in &dirs {
        let mut new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        while valid_pos(new_pos, lines) {
            if lines[new_pos.0 as usize].chars().nth(new_pos.1 as usize) == Some('L') {
                break;
            } 
            if lines[new_pos.0 as usize].chars().nth(new_pos.1 as usize) == Some('#') {
                result += 1;
                break;
            }
            new_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
        }
    }

    result
}

fn iteration(lines : Vec<String>) -> (Vec<String>, bool, i32) {
    let mut result : Vec<String> = vec!["".to_string(); lines.len()];
    let mut changed = false;
    let mut occupied = 0;

    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            let chr = line.chars().nth(j);
            let adj_count = adjacent_count((i as i32, j as i32), &lines);

            let mut target_char = '.';
            if chr == Some('L') {
                if adj_count == 0 {
                    changed = true;
                    target_char = '#';
                } else {
                    target_char = 'L';
                }
            }
            if chr == Some('#') {
                if adj_count >= 5 {
                    changed = true;
                    target_char = 'L';
                } else {
                    target_char = '#';
                }
            }

            if target_char == '#' {
                occupied += 1;
            }

            result[i].push_str(&target_char.to_string());
        }
    }

    (result, changed, occupied)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines : Vec<&str> = contents.split("\n").collect();
    let brute_lines : Vec<String> = lines.iter().map(|x| remove_whitespace(x.trim())).collect();

    let mut new_array = brute_lines;
    let mut cond = true;
    let mut occupied = 0;
    while cond {
        let res = iteration(new_array);
        new_array = res.0; 
        cond = res.1;
        occupied = res.2;
    }

    println!("{}\n", occupied);
}
