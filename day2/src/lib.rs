use std::fs::File;
use std::io::{BufReader, BufRead};

type Operator = fn(u64, u64) -> u64;

fn evaluate(input: &Vec<u64>) -> Vec<u64> {
    let mut result = input.clone();

    for i in (0..result.len()).step_by(4){
        match result[i] {
            1 => {
                handle_add(&mut result, i, add)
            },
            2 =>  handle_add(&mut result, i, mult),
            99 => break,
            _ => eprintln!("Unexpeced instruction!")
        }
    }
    result
}

pub fn test_part_one() {
    let input: Vec<u64> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    let res = evaluate(&input);
    let expected = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
    assert!(res == expected);

    let input = vec![1,0,0,0,99];
    let res = evaluate(&input);
    let expected = vec![2,0,0,0,99];
    assert!(res == expected);

    let input = vec![2,3,0,3,99];
    let res = evaluate(&input);
    let expected = vec![2,3,0,6,99];
    assert!(res == expected);

    let input = vec![2,4,4,5,99,0];
    let res = evaluate(&input);
    let expected: Vec<u64> = vec![2,4,4,5,99,9801];
    assert!(res == expected);

    let input = vec![1,1,1,4,99,5,6,0,99];
    let res = evaluate(&input);
    let expected: Vec<u64> = vec![30,1,1,4,2,5,6,0,99];
    assert!(res == expected);
}


fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mult(a: u64, b:u64) -> u64 {
    a * b
}


fn handle_add(input: &mut Vec<u64>, index: usize, op: Operator) {
    let i1 = input[index + 1];
    let i2 = input[index + 2];
    
    let result_index = input[index + 3] as usize;


    let el1 = input[i1 as usize];
    let el2 = input[i2 as usize];
    

    input[result_index] = op(el1, el2);

}

pub fn part_one(filename: String) -> u64 {
    test_part_one();
    
    let int_codes = parse_file(filename);
    let res = evaluate(&int_codes);
    return res[0]
}

fn parse_file(filename: String) -> Vec<u64> {
    let file = File::open(filename)
        .expect("Can not open file...");

    let mut int_codes: Vec<u64> = Vec::new();
    let buffered = BufReader::new(file);

     for line in buffered.lines() {
         
         match line {
            Ok(l) => {
                for num in l.split(",") {
                    int_codes.push(num.parse::<u64>().unwrap());
                }
            },
            Err(_) => eprintln!("Failed to read line")
         }
     }

     return int_codes;
}

pub fn part_two(filename: String) -> i64 {
    let int_codes = parse_file(filename);
    
    for noun in 0..99 {
        for verb in 0..99 {
            let mut int_codes_clone = int_codes.clone();
            int_codes_clone[1] = noun;
            int_codes_clone[2] = verb;

            let res = evaluate(&int_codes_clone);
            if res[0] == 19690720 {
                let result = 100 * noun + verb;
                return result as i64
            }
        }
    }
    -1
}





