use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_file(filename: String) -> Vec<i64> {
    let file = File::open(filename).expect("Can not open file...");

    let mut int_codes: Vec<i64> = Vec::new();
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        match line {
            Ok(l) => {
                for num in l.split(",") {
                    int_codes.push(num.parse::<i64>().unwrap());
                }
            }
            Err(_) => eprintln!("Failed to read line"),
        }
    }

    return int_codes;
}
