use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_file(filename: String) -> Vec<u32> {
    let file = File::open(filename).expect("Can not open file...");

    let buffered = BufReader::new(file);

    return buffered
        .lines()
        .map(|l| {
            let val: Vec<u32> = l
                .unwrap()
                .chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect();
            return val;
        })
        .flatten()
        .collect();

    /*    for line in buffered.lines() {
        match line {
            Ok(l) => {
                for num in l.chars() {
                    data.push(num.parse::<T>().unwrap());
                }
            }
            Err(_) => eprintln!("Failed to read line"),
        }
    }

    return data;*/
}
