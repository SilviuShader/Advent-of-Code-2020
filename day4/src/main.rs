// TODO: ADD A NEW LINE TO THE INPUT BEFORE STARTING THE PROGRAM
// SHOULD FIX THIS.. BUT MEH

use std::fs;
use std::collections::HashMap;

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn is_valid(current_object : &HashMap<&str, &str>) -> bool {

    for key in &REQUIRED_KEYS {
        if !current_object.contains_key(key) {
            return false;
        }
    }

    true
}

fn string_num_in_range(crt_str : &str, min_val : i32, max_val : i32) -> bool {
    let mut fail = false;
    
    match crt_str.trim().parse::<i32>() {
        Ok(n) if n < min_val || n > max_val => fail = true,
        Ok(_) => fail = fail,
        Err(_) => fail = true
    }

    !fail
}

fn ok_range(current_object : &HashMap<&str, &str>, prop : &str, min_val : i32, max_val : i32) -> bool {
    let fail;

    match current_object.get(prop) {
        Some(st) => fail = !string_num_in_range(st, min_val, max_val),
        None => fail = true 
    }

    !fail
}

fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    if s.ends_with(p) {
        &s[..s.len() - p.len()]
    } else {
        s
    }
}

fn valid_height(height_str : &str) -> bool {
    let used_string = height_str.trim();
    if !used_string.ends_with("cm") && !used_string.ends_with("in") {
        return false;
    }

    let number_str;
    let min_range;
    let max_range;

    if used_string.ends_with("cm") {
        number_str = remove_suffix(used_string, "cm");
        min_range = 150;
        max_range = 193;
    } else {
        number_str = remove_suffix(used_string, "in");
        min_range = 59;
        max_range = 76;
    }

    string_num_in_range(number_str, min_range, max_range)
}

fn valid_hair_color(color_val : &str) -> bool {
    if !color_val.starts_with("#") {
        return false;
    }

    let t = color_val.trim_start_matches("#").trim();

    if t.len() != 6 {
        return false;
    }

    for chr in t.chars() {
        if (chr >= '0' && chr <= '9') ||
           (chr >= 'a' && chr <= 'f'){
           } else {
               return false;
           }
    }

    true
}

fn valid_eye_color(eye_color : &str) -> bool {
    for col in &EYE_COLORS {
        if &eye_color.trim() == col {
            return true;
        }
    }

    false
}

fn valid_pid(pid : &str) -> bool {
    if pid.trim().len() != 9 {
        return false;
    }

    let fail;

    match pid.trim().parse::<i32>() {
        Ok(_) => fail = false,
        Err(_) => fail = true
    }

    !fail
}

fn is_valid2(current_object : &HashMap<&str, &str>) -> bool {

    if !is_valid(current_object) {
        return false;
    }

    if !ok_range(current_object, "byr", 1920, 2002) {
        return false;
    }

    if !ok_range(current_object, "iyr", 2010, 2020) {
        return false;
    }

    if !ok_range(current_object, "eyr", 2020, 2030) {
        return false;
    }

    let mut fail;

    match current_object.get("hgt") {
        Some(st) => fail = !valid_height(st),
        None => fail = true
    }

    if fail {
        return false;
    }

    match current_object.get("hcl") {
        Some(st) => fail = !valid_hair_color(st),
        None => fail = true
    }

    if fail {
        return false
    }

    match current_object.get("ecl") {
        Some(st) => fail = !valid_eye_color(st),
        None => fail = true
    }

    if fail {
        return false
    }
    
    match current_object.get("pid") {
        Some(st) => fail = !valid_pid(st),
        None => fail = true
    }

    !fail
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let entries_strings : Vec<&str> = contents.split("\n").collect();

    let mut current_object = HashMap::new();
    let mut result1 = 0;
    let mut result2 = 0;

    for entry in entries_strings {
        if entry.len() > 1 {
            let elems : Vec<&str> = entry.split(" ").collect();
            for elem in elems {
                let key_val : Vec<&str> = elem.split(":").collect();
                current_object.insert(key_val[0], key_val[1]);
            }
        } 
        else {
            if is_valid(&current_object) {
                result1 += 1;
            }

            if is_valid2(&current_object){
                result2 += 1;
            }

            current_object = HashMap::new();
        }
    }

    println!("{}", result1);
    println!("{}", result2);
}
