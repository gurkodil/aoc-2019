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

pub struct MemHandler {
    raw: Vec<i64>,
    op: usize,
    parameter_modes: usize,
    parameter_offset: usize,
    result: Vec<i64>,
    input: Vec<i64>,
    finished: bool,
}

impl MemHandler {
    pub fn new(raw: Vec<i64>) -> Self {
        return MemHandler {
            raw,
            op: 0,
            parameter_modes: 0,
            parameter_offset: 1,
            result: vec![],
            input: vec![],
            finished: false,
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

    pub fn add_input(&mut self, input: i64) {
        self.input.insert(0, input);
    }

    pub fn get_result(&self) -> Option<i64> {
        if self.result.len() == 0 {
            None
        } else {
            Some(self.result[self.result.len() - 1])
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
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

    pub fn run(&mut self) -> Option<i64> {
        match self.next() {
            1 => {
                // ADD
                let arg0 = self.get_next_parameter();
                let arg1 = self.get_next_parameter();
                let arg2 = self.get_next_pointer();
                self.set_val(arg2, arg0 + arg1);
                self.move_op_by(4);
            }
            2 => {
                // MULTIPLY
                let arg0 = self.get_next_parameter();
                let arg1 = self.get_next_parameter();
                let arg2 = self.get_next_pointer();
                self.set_val(arg2, arg0 * arg1);
                self.move_op_by(4);
            }
            3 => {
                // INPUT
                let arg0 = self.get_next_pointer();
                if let Some(val) = self.input.pop() {
                    self.set_val(arg0, val);
                    self.move_op_by(2);
                }
            }
            4 => {
                // GET OUTPUT
                let arg0 = self.get_next_parameter();
                self.result.push(arg0);
                self.move_op_by(2);
                return Some(arg0);
            }
            5 => {
                // JUMP IF TRUE
                let arg0 = self.get_next_parameter();
                if arg0 != 0 {
                    let arg1 = self.get_next_parameter() as usize;
                    self.move_op(arg1);
                } else {
                    self.move_op_by(3);
                }
            }
            6 => {
                // JUMP IF FALSE
                let arg0 = self.get_next_parameter();
                if arg0 == 0 {
                    let arg1 = self.get_next_parameter() as usize;
                    self.move_op(arg1);
                } else {
                    self.move_op_by(3);
                }
            }
            7 => {
                // LESS THAN
                let arg0 = self.get_next_parameter();
                let arg1 = self.get_next_parameter();
                let arg3 = self.get_next_pointer();
                let val = if arg0 < arg1 { 1 } else { 0 };
                self.set_val(arg3, val);
                self.move_op_by(4);
            }
            8 => {
                // EQUALS
                let arg0 = self.get_next_parameter();
                let arg1 = self.get_next_parameter();
                let arg3 = self.get_next_pointer();
                let val = if arg0 == arg1 { 1 } else { 0 };
                self.set_val(arg3, val);
                self.move_op_by(4);
            }
            // TERMINATE
            99 => self.finished = true,
            other => panic!("Unexpected operation: {}", other),
        }

        return None;
    }
}
