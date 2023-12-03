use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        // Create a regex pattern to match the specified format
        let mut result = 0;
        for (game_id, line) in lines.flatten().enumerate() {
            
            // not all of this is necessary
            let mut nl = line.replace(' ', "").replace("reen", "").replace("lue", "").replace("ed", "");
            let game_id_det_index = line.find(':').unwrap();
            nl.replace_range(..game_id_det_index, "");
            
            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;
            for set in nl.split(';') {

                for value in set.split(',') {
                    println!("{value}");
                    if value.contains('r') {
                        let number = value.split('r').next().unwrap().parse::<i32>().unwrap();
                        if number > red {
                            red = number
                        }
                    } else if value.contains('b') {
                        
                        let number = value.split('b').next().unwrap().parse::<i32>().unwrap();
                        if number > blue {
                            blue = number
                        }
                        
                    } else {
                        let number = value.split('g').next().unwrap().parse::<i32>().unwrap();
                        if number > green {
                            green = number
                        }
                    }
                }

            }
            
            result += blue * red * green;
        }
        println!("{result}");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
