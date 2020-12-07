extern crate regex;

use std::fs;
use std::collections::HashMap;

struct Neighbor {
    freq : u32,
    other_key : String
}

fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    if s.ends_with(p) {
        &s[..s.len() - p.len()]
    } else {
        s
    }
}

fn build_graph(lines : Vec<&str>) -> HashMap<&str, Vec<Neighbor>> {

    let mut graph : HashMap<&str, Vec<Neighbor>> = HashMap::new();

    for line in lines {

        let parts : Vec<&str> = line.split("contain").collect();
        let mut left = parts[0];
        let right = parts[1];
        left = remove_suffix(left, " bags ");
        
        if !graph.contains_key(left) {
            graph.insert(left, Vec::<Neighbor>::new());
        }

        let re = regex::Regex::new(r"bags|bag").unwrap();
        let contents = re.split(right);

        for content in contents {
            if content.len() > 2 {

                let mut trimmed_content = remove_suffix(content, ".");
                trimmed_content = remove_suffix(trimmed_content, " ");
                trimmed_content = trimmed_content.trim_start_matches(" ");
                trimmed_content = trimmed_content.trim_start_matches(",");
                trimmed_content = trimmed_content.trim_start_matches(" ");

                let mut freq : u32 = 0;

                if trimmed_content != "no other" {
                    let details : Vec<&str> = trimmed_content.splitn(2, " ").collect();
                    match details[0].trim().parse::<u32>() {
                        Ok(n) => freq = n,
                        Err(e) => println!("{}", e)
                    }

                    let neigh = Neighbor {
                        freq,
                        other_key : details[1].to_string()
                    };

                    graph.get_mut(left).unwrap().push(neigh); 
                }
            }
        }
    }

    graph
}

fn valid_node1(key : &str, graph : &HashMap::<&str, Vec<Neighbor>>) -> bool {

    let mut result = false;

    for oth in &graph[key] {
        if oth.other_key == "shiny gold" {
            result = true;
        }
        result = result || valid_node1(&oth.other_key, &graph);
    }

    return result;
}

fn count_total_bags(key : &str, graph : &HashMap::<&str, Vec<Neighbor>>) -> u32 {
    let mut result = 0;

    for oth in &graph[key] {
        result += oth.freq * count_total_bags(&oth.other_key, &graph);
    }

    result + 1
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines : Vec<&str> = contents.split("\n").collect();

    let graph = build_graph(lines);
    let mut result1 = 0;

    for (k, _) in &graph {
        if valid_node1(k, &graph) {
            result1 += 1;
        }
    }

    println!("{} {}", result1, count_total_bags("shiny gold", &graph) - 1);
}
