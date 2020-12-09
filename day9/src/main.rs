use std::fs;
use std::collections::HashSet;
use std::cmp::max;
use std::cmp::min;

const PREAMBLE : usize = 25;

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn get_break_position(numbers : &Vec<u64>) -> u64 {
    let mut current_numbers_set = HashSet::new();
    let mut result = 0;

    for ix in 0..numbers.len() {
        if ix >= PREAMBLE {

            let mut found = false;

            for num in &current_numbers_set {
                if numbers[ix] > *num {
                    let diff = numbers[ix] - num;
                    if current_numbers_set.contains(&diff) {
                        found = true;
                    }
                }
            }

            if !found {
                result = numbers[ix];
                break;
            }

            current_numbers_set.remove(&numbers[ix - PREAMBLE]);
        } else {

        }

        current_numbers_set.insert(numbers[ix]);
    }

    result
}

fn solve2(numbers : &Vec<u64>, target : u64) -> u64 {
    let mut sums : Vec<u64> = Vec::new();
    let mut mx = 0;
    let mut mn = std::u64::MAX;
    let mut result = 0;

    for ix in 0..numbers.len() {
        if ix == 0 {
            sums.push(numbers[ix]);
        } else {
            sums.push(sums[ix - 1] + numbers[ix]);
        }
    }

    for sta in 0..numbers.len() {
        for en in sta..numbers.len() {
            let mut fin_sum = sums[en];
            if sta != 0 {
                fin_sum -= sums[sta - 1];
            }

            if fin_sum == target {
                
                for i in sta..en+1 {
                    mx = max(numbers[i], mx);
                    mn = min(numbers[i], mn);
                }
                if result == 0 {
                    result = mx + mn;
                }
            }
        }
    }
    result
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines : Vec<&str> = contents.split("\n").collect();
    let numbers = lines.iter().map(|x| match remove_whitespace(x.trim()).parse::<u64>() {
        Ok(n) => n,
        Err(_) => 0
    }).collect::<Vec<u64>>();

    let result1 = get_break_position(&numbers);

    println!("{}", result1);
    println!("{}", solve2(&numbers, result1));
}
