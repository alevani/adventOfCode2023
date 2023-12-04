use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        let input = lines.flatten().collect();
        part_one(input);
        // part_two(input);
    }
}

fn part_one(input: Vec<String>) {
    let mut result = 0;
    for line in input {
        let mut newline = line.split(": ");
        let _ = newline.next(); // Skips "Card   1: "

        let mut card = newline.next().unwrap().split(" | ");

        let winning_numbers: HashMap<i32, i32> = card
            .next()
            .unwrap()
            .split_whitespace()
            .map(|f| (f.parse::<i32>().unwrap(), 0))
            .collect();
        let my_numbers = card
            .next()
            .unwrap()
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        result += my_numbers
            .iter()
            .map(|n| {
                if winning_numbers.contains_key(n) {
                    2
                } else {
                    1
                }
            })
            .product::<i32>();
    }
    println!("result: {result}");
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
