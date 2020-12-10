use std::fs;

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn update_next(number : u64, index : usize, sorted : &Vec<u64>, occurences : &mut Vec<(u64, u64, u64, u64)>, prev_occ : &(u64, u64, u64, u64)) {

    for ix in index..sorted.len() {
        let crt_val = sorted[ix];

        if crt_val == number + 1 {
            occurences[ix] = (prev_occ.0 + 1, prev_occ.1, prev_occ.2, occurences[ix].3 + prev_occ.3);
        } else if crt_val == number + 2 {
            occurences[ix] = (prev_occ.0, prev_occ.1 + 1, prev_occ.2, occurences[ix].3 + prev_occ.3);
        } else if crt_val == number + 3 {
            occurences[ix] = (prev_occ.0, prev_occ.1, prev_occ.2 + 1, occurences[ix].3 + prev_occ.3);
        } else if crt_val == number {
            occurences[ix] = (prev_occ.0, prev_occ.1, prev_occ.2, occurences[ix].3 + prev_occ.3);
        } else if number > crt_val + 3 {
            break;
        }
    }
}

fn solve(numbers : &Vec<u64>) -> (u64, u64) {
    let mut sorted = numbers.clone();
    sorted.sort();

    let mut occurences : Vec<(u64, u64, u64, u64)> = vec![(0, 0, 0, 0); numbers.len()];

    update_next(0, 0, &sorted, &mut occurences, &(0, 0, 0, 1));

    for ix in 0..sorted.len() {
        let crt_occurences = &occurences[ix].clone();
        update_next(sorted[ix], ix + 1, &sorted, &mut occurences, &crt_occurences);
    }

    let res1 = occurences[sorted.len() - 1].0;
    let res3 = occurences[sorted.len() - 1].2 + 1;

    let res4 = occurences[sorted.len() - 1].3;

    (res1 * res3, res4)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines : Vec<&str> = contents.split("\n").collect();
    let numbers = lines.iter().map(|x| match remove_whitespace(x.trim()).parse::<u64>() {
        Ok(n) => n,
        Err(_) => 0
    }).collect::<Vec<u64>>();

    let result = solve(&numbers);
    println!("{} {}", result.0, result.1);
}
