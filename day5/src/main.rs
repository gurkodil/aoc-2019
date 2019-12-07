mod lib;
mod parser;

fn main() {
    let op_codes = parser::parse_file("data/05.dat".to_string());

    let result = lib::part_one(op_codes.clone());
    println!("Result part 1: {:?}", result);

    let result = lib::part_two(op_codes);
    println!("Result part 2: {:?}", result);
}
