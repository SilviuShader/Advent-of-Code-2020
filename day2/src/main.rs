use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    let mut result1 = 0;
    let mut result2 = 0;

    for line in lines {
        let line_contents : Vec<&str> = line.split(" ").collect();

        let numbers_part : Vec<&str> = line_contents[0].split("-").collect();
    
        let first_number : usize = numbers_part[0].parse::<usize>().unwrap();
        let second_number : usize = numbers_part[1].parse::<usize>().unwrap();

        let target_char = line_contents[1].chars().nth(0).unwrap();
        let mut counter = 0;

        for c in line_contents[2].chars() {
            if c == target_char {
                counter += 1;
            }
        }

        if counter >= first_number && counter <= second_number {
            result1 += 1;
        }

        if (line_contents[2].chars().nth(first_number - 1).unwrap() == target_char && 
            line_contents[2].chars().nth(second_number - 1).unwrap() != target_char) ||
            (line_contents[2].chars().nth(first_number - 1).unwrap() != target_char && 
            line_contents[2].chars().nth(second_number - 1).unwrap() == target_char) {
                result2 += 1;
        }
    }

    println!("{}", result1);
    println!("{}", result2);
}
