use std::fs;

mod lib;

fn main() {
    let input: Vec<i64> = fs::read_to_string("data/09.dat")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|l| l.split(",").map(|c| c.parse::<i64>().unwrap()))
        .flatten()
        .collect();

    println!("Result part one: {:?}", execute(input.clone(), 1));
    println!("Result part two: {:?}", execute(input.clone(), 2));
}

fn execute(codes: Vec<i64>, input: i64) -> Vec<i64> {
    let mut mem_handler = lib::MemHandler::new(codes);
    mem_handler.add_input(input);
    while !mem_handler.finished() {
        mem_handler.run();
    }

    return mem_handler.get_result().unwrap();
}

#[test]
fn test_one() {
    let mut input = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];

    let output = execute(input.clone(), 0);

    assert!(input == output);
}

#[test]
fn test_two() {
    let mut input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

    let output = execute(input.clone(), 0)[0];

    assert!(output < 10000000000000000 && output >= 1000000000000000);
}
#[test]
fn test_three() {
    let mut input = vec![104, 1125899906842624, 99];

    let output = execute(input.clone(), 0)[0];

    assert!(output == 1125899906842624);
}
