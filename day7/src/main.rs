// https://adventofcode.com/2019/day/7
mod lib;
mod parser;
use permutohedron::Heap;

fn main() {
    let res = find_max_signal(&mut [0, 1, 2, 3, 4]);
    println!("Part one: {}", res);
    let res = find_max_signal(&mut [5, 6, 7, 8, 9]);
    println!("Part one: {}", res);
}

fn find_max_signal(phase_setting: &mut [i64; 5]) -> i64 {
    let op_codes = parser::parse_file("data/07.dat".to_string());
    let mut record = 0;
    let perms = Heap::new(phase_setting);

    for perm in perms {
        let res = max_of_single_phase(&perm, op_codes.clone()).unwrap();
        if res > record {
            record = res;
        }
    }
    return record;
}

fn max_of_single_phase(phase_sequence: &[i64; 5], program: Vec<i64>) -> Option<i64> {
    let mut computers = vec![
        lib::MemHandler::new(program.clone()),
        lib::MemHandler::new(program.clone()),
        lib::MemHandler::new(program.clone()),
        lib::MemHandler::new(program.clone()),
        lib::MemHandler::new(program.clone()),
    ];

    for i in 0..computers.len() {
        computers[i].add_input(phase_sequence[i]);
    }

    let mut finished: [bool; 5] = [false; 5];
    let mut prev_val = Some(0);
    while finished.iter().any(|f| *f == false) {
        for i in 0..computers.len() {
            if let Some(val) = prev_val {
                computers[i].add_input(val);
            }
            prev_val = computers[i].run();
            finished[i] = computers[i].finished();
        }
    }

    return computers[4].get_result();
}

#[test]
fn test_examples() {
    let res = max_signal_from_phase(
        &[4, 3, 2, 1, 0],
        parse_str_to_int("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
    );

    assert!(res == 43210);
    let res = max_signal_from_phase(
        &[0, 1, 2, 3, 4],
        parse_str_to_int(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        ),
    );

    assert!(res == 54321);
    let res = max_signal_from_phase(
        &[1, 0, 4, 3, 2],
        parse_str_to_int("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")
    );
    assert!(res == 65210);
}

#[allow(dead_code)]
fn parse_str_to_int(input: &str) -> Vec<i64> {
    input
        .to_string()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}
