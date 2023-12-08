use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        let input = lines.flatten().collect();
        part_one(input);
        // part_two(input);
    }
}

fn part_one(input: Vec<String>) {
    let (directions, nodes) = parse_input(input);
    let bytes = directions.as_bytes();
    let mut step = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let direction: &u8 = bytes.get(step % bytes.len()).unwrap();
        let my_char = char::from_u32(*direction as u32).unwrap();

        current_node = if my_char == 'L' {
            &nodes.get(current_node).unwrap().0
        } else {
            &nodes.get(current_node).unwrap().1
        };

        step += 1;
    }

    println!("result: {step}");
}

fn part_two(input: Vec<String>) {}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let mut clone = input.iter(); // todo look into why I need to do that, and how I can not do that
    let directions = clone.next().unwrap();
    clone.next();

    let pattern = r"\b([A-Z]+)\b";
    let re = Regex::new(pattern).unwrap();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    // Parsing input
    for line in clone {
        let matches: Vec<&str> = re
            .captures_iter(line)
            .filter_map(|capture| capture.get(1).map(|m| m.as_str()))
            .collect();

        nodes.insert(
            matches[0].to_string(),
            (matches[1].to_string(), matches[2].to_string()),
        );
    }

    (directions.to_string(), nodes)
}
