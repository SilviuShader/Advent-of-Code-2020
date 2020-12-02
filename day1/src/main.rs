use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    let mut numbers = Vec::<i32>::new();
    let mut number_set = HashSet::<i32>::new();

    for line in lines {
        let crt_number = line.trim().parse::<i32>().unwrap();
        numbers.push(crt_number);
        number_set.insert(crt_number);
    }

    for number in &numbers {
        let search = 2020 - number;
        if number_set.contains(&search) {
            println!("{}*{}={}", number, search, number * search);
        }
    }

    println!("\n");

    for number in &numbers {
        for number2 in &numbers {
            let sum = number + number2;
            let search = 2020 - sum;
            if number_set.contains(&search) {
                println!("{}", number * number2 * search);
            }
        }
    }
}
