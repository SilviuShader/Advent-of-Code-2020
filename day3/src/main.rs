use std::fs;

fn traverse(lines : &Vec<&str>, (down, right) : (usize, usize)) -> i32 {
    
    let mut start_pos = (0, 0);
    let mut result = 0;
    while start_pos.0 < lines.len() {

        let crt_line = lines[start_pos.0].trim();

        if crt_line.chars().nth(start_pos.1).unwrap() == '#' {
            result += 1;
        }

        start_pos.0 += down;
        start_pos.1 += right;

        start_pos.1 = start_pos.1 % crt_line.len();
    }

    result
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines : Vec<&str> = contents.trim().split("\n").collect();

    let result11 = traverse(&lines, (1, 1));
    let result13 = traverse(&lines, (1, 3));
    let result15 = traverse(&lines, (1, 5));    
    let result17 = traverse(&lines, (1, 7));
    let result21 = traverse(&lines, (2, 1));

    println!("{}", result13);
    println!("{} {} {} {} {}", result11, result13, result15, result17, result21);
}
