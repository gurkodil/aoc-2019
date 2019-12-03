use std::fs::File;
use std::io::prelude::*;
mod lib;

fn main() {
    let mut file = File::open("data/03.dat")
        .expect("Can not open file...");
    
    let mut content = String::new(); 
     
    file.read_to_string(&mut content)
        .expect("Failed reading content from file.");

    let wires = content.split("\n").collect::<Vec<&str>>();
    
    lib::test_part_one();

    if wires.len() < 2 {
        return
    }

    let res_part_one = lib::part_one(
        wires[0].to_string(),
        wires[1].to_string()
        );

    println!("Part one: {}", res_part_one);
}
