// Link to puzzle: https://adventofcode.com/2019/day/5
//
// Summary:
//
// OPCODES:
//  1:  Add instruction, followed by two parameters and lastly the address to
//      where the results should be stored.
//  2:  Multiply instruction, followed by two parameters and lastly the
//      address where the results should be stored.
//  3:  Takes a single integer as input and saves it to the address pointed
//      to by its only parameter.
//  4:  Outputs the value of its only parameter, for example instruction
//      '4, 5' would output the value at adress 50.
//
// PARAMETER MODES:
//  0 -> POSITION_MODE: parameter will be interpreted as a position, for
//     example:
//         parameter 50, the value is stored at adress 50 in memory.
//  1 -> IMMEDIATE_MODE: The parameter value is simply the value that will
//         be interpreted.
//
// INSTSTRUCTIONS FORMAT:
//    Parameter mode is stored in the same value as the instruction's
//    opcode.
//
//    OPCODE: The opcode is a two-digit number based only on the ones and
//    tens digits of the value.
//
// PARAMETERS:
//     Parameter modes are single digits, one per parameter, read right-
//     to-left from the opcode: the first parameter's mode is in the hundreds
//     digit, the second parameter's mode is in the thousands digit, the third
//     parameter's mode is in the ten-thousands digit, and so on.
//     Missing modes are 0.
//
// EXAMPLE PROGRAM: '1002,4,3,4,33'
//     1002 gives the following instructions:
//         02 -> opcode 2 which is multiply instruction
//         0  -> position mode for parameter 1
//         1  -> immediate mode for parameter 2
//
//     This will result in position mode for first parameter, 4. Which points
//     to the value 33. The second parameter is in immediate mode which
//     gives the second parameter a value of 3.
//
//     The last value 4 is where we store the result (33*3=99), which is in
//     position 4. Executing the program will result as following:
//
//         '1002, 4, 3, 4, 99'
//
// FINALLY:
//     - Should take an input parameter
//     - Length of the instructions varies,depending on the instruction
//       format.
//

struct MemHandler {
    raw: Vec<i64>,
    op: usize,
    parameter_modes: usize,
    parameter_offset: usize,
}

impl MemHandler {
    pub fn new(raw: Vec<i64>) -> Self {
        return MemHandler {
            raw,
            op: 0,
            parameter_modes: 0,
            parameter_offset: 1,
        };
    }

    pub fn next(&mut self) -> usize {
        let next = (self.raw[self.op] % 100) as usize;
        self.parameter_modes = (self.raw[self.op] / 100) as usize;
        self.parameter_offset = 1;
        return next;
    }

    pub fn set_val(&mut self, address: usize, val: i64) {
        self.raw[address] = val;
    }

    pub fn move_op(&mut self, to: usize) {
        self.op = to;
    }

    pub fn move_op_by(&mut self, steps: usize) {
        self.op += steps;
    }

    pub fn get_next_pointer(&self) -> usize {
        return self.raw[self.op + self.parameter_offset] as usize;
    }

    pub fn get_next_parameter(&mut self) -> i64 {
        let parameter_mode = self.parameter_modes % 10;
        let address = self.raw[self.op + self.parameter_offset];
        self.parameter_modes /= 10;
        self.parameter_offset += 1;
        if parameter_mode == 1 {
            return address;
        } else {
            return self.raw[address as usize];
        }
    }
}

fn run_diag(input: Vec<i64>, diagnostics_id: i64) -> Vec<i64> {
    let mut mem_handler = MemHandler::new(input.clone());
    let mut output = Vec::new();

    loop {
        match mem_handler.next() {
            1 => {
                // ADD
                let arg0 = mem_handler.get_next_parameter();
                let arg1 = mem_handler.get_next_parameter();
                let arg2 = mem_handler.get_next_pointer();
                mem_handler.set_val(arg2, arg0 + arg1);
                mem_handler.move_op_by(4);
            }
            2 => {
                // MULTIPLY
                let arg0 = mem_handler.get_next_parameter();
                let arg1 = mem_handler.get_next_parameter();
                let arg2 = mem_handler.get_next_pointer();
                mem_handler.set_val(arg2, arg0 * arg1);
                mem_handler.move_op_by(4);
            }
            3 => {
                // SET INPUT AT VAL
                let arg0 = mem_handler.get_next_pointer();
                mem_handler.set_val(arg0, diagnostics_id);
                mem_handler.move_op_by(2);
            }
            4 => {
                // GET OUTPUT
                let arg0 = mem_handler.get_next_parameter();
                output.push(arg0);
                mem_handler.move_op_by(2);
            }
            5 => {
                // JUMP IF TRUE
                let arg0 = mem_handler.get_next_parameter();
                if arg0 != 0 {
                    let arg1 = mem_handler.get_next_parameter() as usize;
                    mem_handler.move_op(arg1);
                } else {
                    mem_handler.move_op_by(3);
                }
            }
            6 => {
                // JUMP IF FALSE
                let arg0 = mem_handler.get_next_parameter();
                if arg0 == 0 {
                    let arg1 = mem_handler.get_next_parameter() as usize;
                    mem_handler.move_op(arg1);
                } else {
                    mem_handler.move_op_by(3);
                }
            }
            7 => {
                // LESS THAN
                let arg0 = mem_handler.get_next_parameter();
                let arg1 = mem_handler.get_next_parameter();
                let arg3 = mem_handler.get_next_pointer();
                let val = if arg0 < arg1 { 1 } else { 0 };
                mem_handler.set_val(arg3, val);
                mem_handler.move_op_by(4);
            }
            8 => {
                // EQUALS
                let arg0 = mem_handler.get_next_parameter();
                let arg1 = mem_handler.get_next_parameter();
                let arg3 = mem_handler.get_next_pointer();
                let val = if arg0 == arg1 { 1 } else { 0 };
                mem_handler.set_val(arg3, val);
                mem_handler.move_op_by(4);
            }
            // TERMINATE
            99 => break,
            other => panic!("Unexpected operation: {}", other),
        }
    }

    return output;
}

pub fn part_one(int_codes: Vec<i64>) -> Vec<i64> {
    run_diag(int_codes, 1)
}

pub fn part_two(int_codes: Vec<i64>) -> Vec<i64> {
    run_diag(int_codes, 5)
}
