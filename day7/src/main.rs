// https://adventofcode.com/2019/day/7
mod lib;
mod parser;

fn main() {
    let res = part_one();
    println!("Part one: {}", res);
}

fn part_one() -> i64 {
    let op_codes = parser::parse_file("data/07.dat".to_string());
    let mut phase_sequence = [0; 5];
    let mut record = 0;

    // Testing all combinations of phase settings
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        phase_sequence[0] = a;
                        phase_sequence[1] = b;
                        phase_sequence[2] = c;
                        phase_sequence[3] = d;
                        phase_sequence[4] = e;

                        // All phases have to be unique
                        if !(1..phase_sequence.len())
                            .any(|i| phase_sequence[i..].contains(&phase_sequence[i - 1]))
                        {
                            let res = max_signal_from_phase(&phase_sequence, op_codes.clone());
                            if res > record {
                                record = res;
                            }
                        }
                    }
                }
            }
        }
    }
    return record;
}

fn max_signal_from_phase(phase_sequence: &[i64; 5], program: Vec<i64>) -> i64 {
    let mut input = vec![0];
    for i in 0..phase_sequence.len() {
        input.push(phase_sequence[i]);
        let res = lib::run_diag(program.clone(), &mut input);
        input.push(res[0]);
    }

    return input.pop().unwrap();
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
