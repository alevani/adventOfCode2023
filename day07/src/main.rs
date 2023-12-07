use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        let input: Vec<String> = lines.flatten().collect();
        let mut hand_and_bid: Vec<PokerMap> = Vec::new();
        for line in input {
            let mut split = line.split(' ');
            let hand = split.next().unwrap().to_string();
            let bid = split.next().unwrap().parse::<i32>().unwrap();
            hand_and_bid.push(PokerMap { hand, bid, value_map: "".to_string() });
        }
        part_one(&mut hand_and_bid);
        // part_two(input);
    }
}


#[derive(Debug)]
struct PokerMap {
    pub hand: String,
    pub bid: i32,
    pub value_map: String,
}

impl PokerMap {
    pub fn set_value_map(&mut self, m: String) {
        self.value_map = m;
    } 
}

// Define a custom comparator function
fn custom_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let order = "AKQJT98765432";
    let get_rank = |ch| order.find(ch);

    for (char_a, char_b) in a.chars().zip(b.chars()) {
        if let (Some(rank_a), Some(rank_b)) = (get_rank(char_a), get_rank(char_b)) {
            match rank_a.cmp(&rank_b) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
    }

    // If one string is a prefix of the other, the shorter one comes first
    a.len().cmp(&b.len())
}

fn part_one(input: &mut Vec<PokerMap>) {
    let mut memory: Vec<Vec<&mut PokerMap>> = vec![vec![],vec![],vec![],vec![],vec![],vec![]];

    for pm in input {
        let mut map = HashMap::new();
        map.insert('A', 0);
        map.insert('K', 0);
        map.insert('Q', 0);
        map.insert('J', 0);
        map.insert('T', 0);
        map.insert('9', 0);
        map.insert('8', 0);
        map.insert('7', 0);
        map.insert('6', 0);
        map.insert('5', 0);
        map.insert('4', 0);
        map.insert('3', 0);
        map.insert('2', 0);

        pm.hand.chars().for_each(|c| {
            map.entry(c).and_modify(|value| *value += 1);
        });

        let value_map = map.values();
        let max = value_map.clone().max().unwrap();
        
        let mapping = value_map.cloned().map(|a| a.to_string()).collect::<Vec<String>>().join("");
        pm.set_value_map(mapping);

        memory[*max].push(pm);
    }
    memory.iter_mut().for_each(|hands| hands.sort_by(|a, b| custom_compare(&a.hand, &b.hand)));
    
    let mut result = 0;
    for (index, pm) in memory.iter().flatten().enumerate() {
        println!("{:?}", pm.hand);
        println!("{:?} * {:?}", index + 1 , pm.bid);
        result += (index as i32 + 1) * pm.bid;
    }

    println!("{result:?}");
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
