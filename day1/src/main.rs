use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("puzzle_input.txt")
        .expect("Can not open file...");
    
    let module_masses = read_input(file);

    first_puzzle(&module_masses);
    second_puzzle(&module_masses);
}

fn first_puzzle(data: &Vec<i32>) {
    let total_fuel: i32 = data.iter()
        .map(|mass| mass / 3 - 2)
        .fold(0, |acc, x| acc + x);

    println!("First puzzle:\t[{}]", total_fuel);
}

fn second_puzzle(data: &Vec<i32>) {
     let total_fuel: i32 = data.iter()
        .map(|mass| mass / 3 - 2)
        .map(|fuel| fuels_fuel(fuel))
        .fold(0, |acc, x| acc + x);

    println!("Second puzzle:\t[{}]", total_fuel);
}

fn read_input(file: File) -> Vec<i32> {
    let buffered = BufReader::new(file);
    buffered.lines()
        .map(|l| l.expect("Could not read line"))
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn fuels_fuel(fuel: i32) -> i32 {
   if fuel < 8 {
       fuel
   } else {
       fuel + fuels_fuel(fuel / 3 - 2)
   }
}

