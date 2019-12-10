use std::fs::File;
use std::io::{BufRead, BufReader};
mod graph;
mod lib;

fn main() {
    let vals = parse_file("data/06.dat".to_string());

    let checksum = lib::part_one(&vals);
    println!("Part one (checksum): {}", checksum);
    let min_orbit = lib::part_two(&vals);
    println!("Part two (min orbi): {}", min_orbit);
}

fn parse_file(filename: String) -> Vec<(String, String)> {
    let file = File::open(filename).expect("Can not open file...");
    let mut connections: Vec<(String, String)> = Vec::new();
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        match line {
            Ok(l) => {
                connections.push(parse_node_string(l));
            }
            Err(_) => eprintln!("Failed to read line"),
        }
    }
    return connections;
}

fn parse_node_string(value: String) -> (String, String) {
    let mut vals = value.split(")");
    (
        vals.next().unwrap().to_string(),
        vals.next().unwrap().to_string(),
    )
}
