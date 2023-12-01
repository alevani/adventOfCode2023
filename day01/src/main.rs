use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut sum = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut sl = false;
            let mut sr = false;
            let l = line.unwrap();
            let c = l.chars().collect::<Vec<char>>();
            let mut holder = "".to_string();

            for i in 0..c.len() {
                if sl && sr {
                    break;
                }

                if c[i].is_numeric() && !sl {
                    holder.insert_str(0, &c[i].to_string());

                    sl = true;
                }

                if c[c.len() - 1 - i].is_numeric() && !sr {
                    holder.push_str(&c[c.len() - 1 - i].to_string());

                    sr = true;
                }
            }

            sum += holder.parse::<i32>().unwrap();
        }
    }

    println!("{sum}");
}

// Unfortunately, heavily copied from another answer.
fn part_two() {
    let mut numbers: HashMap<&str, u32> = HashMap::new();
    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    numbers.insert("four", 4);
    numbers.insert("five", 5);
    numbers.insert("six", 6);
    numbers.insert("seven", 7);
    numbers.insert("eight", 8);
    numbers.insert("nine", 9);
    numbers.insert("1", 1);
    numbers.insert("2", 2);
    numbers.insert("3", 3);
    numbers.insert("4", 4);
    numbers.insert("5", 5);
    numbers.insert("6", 6);
    numbers.insert("7", 7);
    numbers.insert("8", 8);
    numbers.insert("9", 9);

    let mut sum = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut first_char = 0;
            let mut last_char = 0;
            let l = line.unwrap();

            let mut si = 1000;
            let mut bg = 0;

            for value in numbers.iter() {
                if let Some(index) = l.find(value.0) {
                    if index < si {
                        first_char = *value.1;
                        si = index;
                    }
                }

                if let Some(index) = l.rfind(value.0) {
                    if index >= bg {
                        bg = index;
                        last_char = *value.1;
                    }
                }
            }

            sum += (first_char * 10) + last_char;
        }
    }

    println!("{sum}");
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
