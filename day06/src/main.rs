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
    let times = [48, 93, 84, 66];
    let distances = vec![261, 1192, 1019, 1063];

    let mut ways = Vec::new();
    for (time, dist) in times.iter().zip(distances) {
        let mut way = 0;
        for i in 1..time + 1 {
            if i * (time - i) >= dist {
                way += 1;
            }
        }
        ways.push(way);
    }

    println!("{}", ways.iter().product::<i32>());
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
