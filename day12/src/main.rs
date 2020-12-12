use std::fs;

struct Vector2(i32, i32);

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn solve(lines : &Vec<String>) -> i32 {
    
    let mut crt_position = Vector2(0, 0);
    let directions = [ Vector2(1, 0), Vector2(0, -1), Vector2(-1, 0), Vector2(0, 1) ];
    let mut crt_direction : i32 = 0;

    for line in lines {
        let fst = &line[0..1];
        let chr = fst.chars().nth(0);
        let num_str = &line.to_string()[1..];
        let mut num = 0;
        match num_str.parse::<i32>() {
            Ok(n) => num = n,
            Err(e) => println!("{}", e)
        }

        match chr {
            Some('N') => { crt_position.1 += num; },
            Some('S') => { crt_position.1 -= num; },
            Some('E') => { crt_position.0 += num; },
            Some('W') => { crt_position.0 -= num; },
            Some('L') => {
                match num {
                    90 => crt_direction -= 1,
                    180 => crt_direction -= 2,
                    270 => crt_direction -= 3,
                    _ => println!("WTF")
                }
                while crt_direction < 0 {
                    crt_direction += 4;
                }
            },
            Some('R') => {
                match num {
                    90 => crt_direction += 1,
                    180 => crt_direction += 2,
                    270 => crt_direction += 3,
                    _ =>println!("WTF2")
                }
                while crt_direction >= 4 {
                    crt_direction -= 4;
                }
            },
            Some('F') => {
                crt_position = Vector2(crt_position.0 + directions[crt_direction as usize].0 * num, 
                    crt_position.1 + directions[crt_direction as usize].1 * num)
            },
            Some(_) => {},
            None => {}
        }
    }

    i32::abs(crt_position.0) + i32::abs(crt_position.1)
}

fn solve2(lines : &Vec<String>) -> i32 {
    
    let mut crt_position = Vector2(0, 0);
    let mut crt_waypoint = Vector2(10, 1);

    for line in lines {
        let fst = &line[0..1];
        let chr = fst.chars().nth(0);
        let num_str = &line.to_string()[1..];
        let mut num = 0;
        match num_str.parse::<i32>() {
            Ok(n) => num = n,
            Err(e) => println!("{}", e)
        }

        match chr {
            Some('N') => { crt_waypoint.1 += num; },
            Some('S') => { crt_waypoint.1 -= num; },
            Some('E') => { crt_waypoint.0 += num; },
            Some('W') => { crt_waypoint.0 -= num; },
            Some('L') => {
                let old_waypoint = Vector2(crt_waypoint.0, crt_waypoint.1);
                match num {
                    90 => crt_waypoint = Vector2(-old_waypoint.1, old_waypoint.0),
                    180 => crt_waypoint = Vector2(-old_waypoint.0, -old_waypoint.1),
                    270 => crt_waypoint = Vector2(old_waypoint.1, -old_waypoint.0),
                    _ => println!("WTF")
                }
            },
            Some('R') => {
                let old_waypoint = Vector2(crt_waypoint.0, crt_waypoint.1);
                match num {
                    90 => crt_waypoint = Vector2(old_waypoint.1, -old_waypoint.0),
                    180 => crt_waypoint = Vector2(-old_waypoint.0, -old_waypoint.1),
                    270 => crt_waypoint = Vector2(-old_waypoint.1, old_waypoint.0),
                    _ =>println!("WTF2")
                }
            },
            Some('F') => {
                crt_position = Vector2(crt_position.0 + crt_waypoint.0 * num, 
                    crt_position.1 + crt_waypoint.1 * num)
            },
            Some(_) => {},
            None => {}
        }
    }

    i32::abs(crt_position.0) + i32::abs(crt_position.1)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines : Vec<&str> = contents.split("\n").collect();
    let brute_lines : Vec<String> = lines.iter().map(|x| remove_whitespace(x.trim())).collect();

    println!("{}", solve(&brute_lines));
    println!("{}", solve2(&brute_lines));
}
