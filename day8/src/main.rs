use std::fs;

fn execute_program(prog_code : &Vec<&str>, change : Option<usize>) -> Vec<usize> {
    let mut current_line : usize = 0;
    let mut already_been = vec![false; prog_code.len()];
    let mut accumulator = 0;
    let mut potential_changes = Vec::<usize>::new();

    while current_line < prog_code.len() && !already_been[current_line] {
        already_been[current_line] = true;

        let proc_args : Vec<&str> = prog_code[current_line].split(" ").collect();
        let mut instr_name = proc_args[0];

        if change != None {
            if current_line == change.unwrap() {
                match proc_args[0] {
                    "jmp" => instr_name = "nop",
                    "nop" => instr_name = "jmp",
                    &_ => instr_name = "error"
                }
            }
        }

        current_line += 1;
        let sec_arg;
        match proc_args[1].trim().parse::<i32>() {
            Ok(n) => sec_arg = n,
            Err(e) => {
                sec_arg = 0;
                println!("{}", e);
            }
        }
        match instr_name {
            "nop" => {
                potential_changes.push(current_line - 1);
            },
            "acc" => {
                accumulator += sec_arg;
            },
            "jmp" => {
                potential_changes.push(current_line - 1);
                current_line = ((current_line as i32) + sec_arg - 1) as usize;
            },
            &_ => {
            }
        }
    }

    if current_line >= prog_code.len() {    
        println!("{}", accumulator);
    }

    potential_changes
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines : Vec<&str> = contents.split("\n").collect();
    let potential_changes = execute_program(&lines, None);

    for change in potential_changes {
        execute_program(&lines, Some(change));
    }
}
