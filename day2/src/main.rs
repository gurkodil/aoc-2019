mod lib;

fn main() { 
    lib::test_part_one();

    println!("Test succeded!");
    let file_name = "data/02.dat".to_string();

    let result_part_one = lib::part_one(file_name.clone());
    let result_part_two = lib::part_two(file_name.clone());

    println!("Result part one: {}", result_part_one);
    println!("Result part two: {}", result_part_two);
}
